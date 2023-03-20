use super::Device;
use crate::TrapCause;

const PRIORITY_BASE: usize = 0x0;
const PRIORITY_PER_ID: usize = 0x4;
const ENABLE_BASE: usize = 0x2000;
const ENABLE_PER_HART: usize = 0x80;
const CONTEXT_BASE: usize = 0x200000;
const CONTEXT_PER_HART: usize = 0x1000;
const CONTEXT_THRESHOLD: usize = 0x0;
const CONTEXT_CLAIM: usize = 0x0;

const PLIC_SIZE: usize = 0x0100_0000;

pub struct Plic {
    priority: Vec<u8>,
    level: Vec<u32>,
    enable: Vec<u32>,
    pending: Vec<u32>,
    pending_priority: Vec<u8>,
    claimed: Vec<u32>,
    pub base_addr: u64,
    size: usize,
}

impl Default for Plic {
    fn default() -> Self {
        Self::new()
    }
}

impl Plic {
    #[allow(arithmetic_overflow)]
    pub fn new() -> Self {
        const PLIC_MAX_DEVICES: usize = 1024;
        Plic {
            priority: vec![0; PLIC_MAX_DEVICES],
            level: vec![0; PLIC_MAX_DEVICES],
            enable: vec![0; PLIC_MAX_DEVICES / 32],
            pending: vec![0; PLIC_MAX_DEVICES / 32],
            pending_priority: vec![0; PLIC_MAX_DEVICES],
            claimed: vec![0; PLIC_MAX_DEVICES / 32],
            base_addr: 0x0c00_0000,
            size: PLIC_SIZE,
        }
    }

    fn priority_read(&self, offset: u64) -> u32 {
        let index = (offset >> 2) as usize;
        if index > 0 && index < PLIC_SIZE {
            self.priority[index] as u32
        } else {
            0
        }
    }

    fn priority_write(&self, offset: u64, val: u32) {
        const PLIC_PRIO_MASK: u32 = 0b1111;
        let index = (offset >> 2) as usize;
        if index > 0 && index < PLIC_SIZE {
            self.priority[index] = (val & PLIC_PRIO_MASK) as u8;
        }
    }

    pub fn context_update(&self) {
        //let best_id = context_best_pending();
        //let mask = ;
    }
}

#[allow(clippy::identity_op)]
impl Device for Plic {
    // is addr in device address space
    fn in_range(&self, addr: u64) -> bool {
        (self.base_addr..=self.base_addr + self.size as u64).contains(&addr)
    }

    // address to raw index
    fn addr2index(&self, addr: u64) -> usize {
        (addr - self.base_addr) as usize
    }

    // store
    fn store8(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "plic only allows load/store32,64 but try store8".to_string(),
        ))
    }

    fn store16(&mut self, addr: u64, _data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::StoreAMOPageFault,
            "plic only allows load/store32,64 but try store16".to_string(),
        ))
    }

    fn store32(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.plic[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.plic[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.plic[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.plic[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    fn store64(&mut self, addr: u64, data: u64) -> Result<(), (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        self.plic[addr + 7] = ((data >> 56) & 0xFF) as u8;
        self.plic[addr + 6] = ((data >> 48) & 0xFF) as u8;
        self.plic[addr + 5] = ((data >> 40) & 0xFF) as u8;
        self.plic[addr + 4] = ((data >> 32) & 0xFF) as u8;
        self.plic[addr + 3] = ((data >> 24) & 0xFF) as u8;
        self.plic[addr + 2] = ((data >> 16) & 0xFF) as u8;
        self.plic[addr + 1] = ((data >> 8) & 0xFF) as u8;
        self.plic[addr + 0] = ((data >> 0) & 0xFF) as u8;
        Ok(())
    }

    // load
    fn load8(&mut self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load8".to_string(),
        ))
    }

    fn load16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load16".to_string(),
        ))
    }

    fn load32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok(((self.plic[addr + 3] as i32) << 24
            | (self.plic[addr + 2] as i32) << 16
            | (self.plic[addr + 1] as i32) << 8
            | (self.plic[addr + 0] as i32)) as i64 as u64)
    }

    fn load64(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        let addr = self.addr2index(addr);
        Ok((self.plic[addr + 7] as u64) << 56
            | (self.plic[addr + 6] as u64) << 48
            | (self.plic[addr + 5] as u64) << 40
            | (self.plic[addr + 4] as u64) << 32
            | (self.plic[addr + 3] as u64) << 24
            | (self.plic[addr + 2] as u64) << 16
            | (self.plic[addr + 1] as u64) << 8
            | (self.plic[addr + 0] as u64))
    }

    fn load_u8(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load_u8".to_string(),
        ))
    }

    fn load_u16(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load_u16".to_string(),
        ))
    }

    fn load_u32(&self, addr: u64) -> Result<u64, (Option<u64>, TrapCause, String)> {
        Err((
            Some(addr),
            TrapCause::LoadPageFault,
            "plic only allows load/store32,64 but try load_u32".to_string(),
        ))
    }
}
