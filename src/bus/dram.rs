use crate::{elfload, TrapCause};
use super::Device;

pub struct Dram {
        dram: Vec<u8>,
    pub base_addr: u32,
        size: usize,
}

impl Dram {
    pub fn new(loader: elfload::ElfLoader) -> Dram {
        const DRAM_SIZE: usize = 1024 * 1024 * 128; // 2^27
        let virt_entry = loader
            .get_entry_point()
            .expect("entry point not found.");

        // create new dram 
        let mut new_dram = vec![0; DRAM_SIZE];

        // load elf memory mapping 
        for segment in loader.prog_headers.iter() {
            if segment.is_loadable() {
                let dram_start = (segment.p_paddr - virt_entry) as usize;
                let mmap_start = (segment.p_offset) as usize;
                let dram_end = dram_start + segment.p_filesz as usize;
                let mmap_end = (segment.p_offset + segment.p_filesz) as usize;

                new_dram.splice(
                    dram_start .. dram_end,
                    loader.mem_data[mmap_start .. mmap_end].iter().cloned()
                );
            }
        }

        let dram_size = new_dram.len();
        Dram {
            dram: new_dram,
            base_addr: virt_entry,
            size: dram_size,
        }
    }
}

#[allow(clippy::identity_op)]
impl Device for Dram {
    // is addr in device address space
    fn in_range(&self, addr: u32) -> bool {
        (self.base_addr ..= self.base_addr + self.size as u32).contains(&addr)
    }

    // address to raw index
    fn addr2index(&self, addr: u32) -> usize {
        (addr - self.base_addr) as usize
    }

    // get 1 byte
    fn raw_byte(&self, addr: u32) -> u8 {
        let addr = self.addr2index(addr);
        self.dram[addr]
    }

    // store
    fn store8(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.dram[addr] = (data & 0xFF) as u8;
        Ok(())
    }

    fn store16(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.dram[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store32(&mut self, addr: u32, data: u32) -> Result<(), (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.dram[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >>  0) & 0xFF) as u8;
        Ok(())
    }

    fn store64(&mut self, addr: u32, data: i64) -> Result<(), (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.dram[addr + 7] = ((data >> 56) & 0xFF) as u8;
        self.dram[addr + 6] = ((data >> 48) & 0xFF) as u8;
        self.dram[addr + 5] = ((data >> 40) & 0xFF) as u8;
        self.dram[addr + 4] = ((data >> 32) & 0xFF) as u8;
        self.dram[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.dram[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.dram[addr + 1] = ((data >>  8) & 0xFF) as u8;
        self.dram[addr + 0] = ((data >>  0) & 0xFF) as u8;
        Ok(())
    }

    // load
    fn load8(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(self.dram[addr] as i8 as i32 as u32)
    }

    fn load16(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((
         (self.dram[addr + 1] as i16) << 8 |
         (self.dram[addr + 0] as i16)
        ) as i32 as u32)
    }

    fn load32(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((
         (self.dram[addr + 3] as u32) << 24 |
         (self.dram[addr + 2] as u32) << 16 |
         (self.dram[addr + 1] as u32) <<  8 |
         (self.dram[addr + 0] as u32)
        ))
    }

    fn load64(&self, addr: u32) -> Result<u64, (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((
         (self.dram[addr + 7] as u64) << 56 |
         (self.dram[addr + 6] as u64) << 48 |
         (self.dram[addr + 5] as u64) << 40 |
         (self.dram[addr + 4] as u64) << 32 |
         (self.dram[addr + 3] as u64) << 24 |
         (self.dram[addr + 2] as u64) << 16 |
         (self.dram[addr + 1] as u64) <<  8 |
         (self.dram[addr + 0] as u64)
        ))
    }

    fn load_u8(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(self.dram[addr] as u32)
    }

    fn load_u16(&self, addr: u32) -> Result<u32, (Option<u32>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((
         (self.dram[addr + 1] as u16) << 8 |
         (self.dram[addr + 0] as u16)
        ) as u32)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    const DRAM_SIZE: usize = 1024 * 1024 * 128; // 2^27

    #[test]
    fn load_store_u8_test() {
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE], base_addr: 0, size: DRAM_SIZE };
        let mut addr = 0;
        let mut test_8 = |data: u32| {
            Dram::store8(dram, addr, data).unwrap();
            assert_eq!(data, Dram::load_u8(dram, addr).unwrap());
            addr += 2;
        };

        test_8(0);
        test_8(17);
        test_8(0b01111111);

        Dram::store8(dram, addr, -42).unwrap();
        assert_ne!(-42, Dram::load_u8(dram, addr).unwrap());
        Dram::store16(dram, addr, -42).unwrap();
        assert_eq!(214, Dram::load_u8(dram, addr).unwrap());
    }

    #[test]
    fn load_store_8_test() {
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE], base_addr: 0, size: DRAM_SIZE };
        let mut addr = 0;
        let mut test_8 = |data: u32| {
            Dram::store8(dram, addr, data).unwrap();
            assert_eq!(data, Dram::load8(dram, addr).unwrap());
            addr += 2;
        };

        test_8(0);
        test_8(17);
        test_8(0b01111111);
        test_8(-42);

        Dram::store8(dram, addr, 0b10000000).unwrap();
        assert_ne!(0b10000000, Dram::load8(dram, addr).unwrap());
    }

    #[test]
    fn load_store_16_test() {
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE], base_addr: 0, size: DRAM_SIZE };
        let mut addr = 0;
        let mut test_16 = |data: u32| {
            Dram::store16(dram, addr, data).unwrap();
            assert_eq!(data, Dram::load16(dram, addr).unwrap());
            addr += 2;
        };

        test_16(0);
        test_16(157);
        test_16(255);
        test_16(-42);
        test_16(0b0111111111111111);

        Dram::store16(dram, addr, 0b1000000010000000).unwrap();
        assert_ne!(0b1000000010000000, Dram::load16(dram, addr).unwrap());
    }

    #[test]
    fn load_store_u16_test() {
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE], base_addr: 0, size: DRAM_SIZE };
        let mut addr = 0;
        let mut test_u16 = |data: u32| {
            Dram::store16(dram, addr, data).unwrap();
            assert_eq!(data, Dram::load_u16(dram, addr).unwrap());
            addr += 2;
        };

        test_u16(0);
        test_u16(157);
        test_u16(255);
        test_u16(0b0111111111111111);

        Dram::store16(dram, addr, -42).unwrap();
        assert_ne!(-42, Dram::load_u16(dram, addr).unwrap());
        Dram::store16(dram, addr, -42).unwrap();
        assert_eq!(65494, Dram::load_u16(dram, addr).unwrap());
    }

    #[test]
    #[allow(overflowing_literals)]
    fn load_store_32_test() {
        let dram = &mut Dram{ dram: vec![0; DRAM_SIZE], base_addr: 0, size: DRAM_SIZE };
        let mut addr = 0;
        let mut test_32 = |data: u32| {
            Dram::store32(dram, addr, data).unwrap();
            assert_eq!(data, Dram::load32(dram, addr).unwrap());
            addr += 2;
        };

        test_32(0);
        test_32(157);
        test_32(255);
        test_32(-42);
        test_32(0b1000000010000000);
        test_32(0b10000000100000001000000010000000);
    }
}
