use crate::bus::Device;
use crate::bus::dram::Dram;
use crate::cpu::PrivilegedLevel;
use dbg_hex::dbg_hex;

pub enum AddrTransMode {
    Bare,
    Sv32,
}

pub struct MMU {
    ppn: usize,
    trans_mode: AddrTransMode,
}

impl MMU {
    pub fn new() -> MMU {
        MMU {
            ppn: 0,
            trans_mode: AddrTransMode::Bare,
        }
    }

    fn update_data(&mut self, satp: u32) {
        self.ppn = (satp & 0x3FFFFF) as usize;
        self.trans_mode = match satp >> 31 & 0x1 {
            1 => AddrTransMode::Sv32,
            _ => AddrTransMode::Bare,
        };
    }

    fn check_pte_validity(&self, pte: i32) -> Result<usize, ()>{
        let pte_v = pte & 0x1;
        let pte_r = pte >> 1 & 0x1;
        let pte_w = pte >> 2 & 0x1;

        if pte_v == 0 || (pte_r == 0 && pte_w == 1) {
            Err(())
        } else {
            Ok(pte as usize)
        }
    }

    fn check_leaf_pte(&self, pte: usize) -> bool {
        let pte_r = pte >> 1 & 0x1;
        let pte_x = pte >> 3 & 0x1;

        pte_r == 1 || pte_x == 1
    }

    #[allow(non_snake_case)]
    pub fn trans_addr(&mut self, addr: usize, satp: u32, 
                      dram: &Dram, priv_lv: &PrivilegedLevel) -> Result<usize, ()> {
        const PTESIZE: usize = 4;
        const PAGESIZE: usize = 4096; // 2^12

        self.update_data(satp);

        match priv_lv {
            PrivilegedLevel::Supervisor |
            PrivilegedLevel::User => {
                match self.trans_mode {
                    AddrTransMode::Bare => Ok(addr),
                    AddrTransMode::Sv32 => {
                        let VPN1 = addr >> 22 & 0x3FF;
                        let VPN0 = addr >> 12 & 0x3FF;
                        let page_off = addr & 0xFFF;

                        // first table walk
                        dbg_hex!(satp);
                        dbg_hex!(self.ppn);
                        dbg_hex!(VPN1);
                        let PTE_addr = self.ppn * PAGESIZE + VPN1 * PTESIZE;
                        let PTE = match self.check_pte_validity(dram.load32(PTE_addr)) {
                            Ok(pte) => pte,
                            Err(()) => {
                                return Err(()) // exception
                            },
                        };
                        let PPN1 = (PTE >> 20 & 0xFFF) as usize;
                        println!("PTE_addr(1): 0x{:x}", PTE_addr);
                        println!("PTE(1): 0x{:x}", PTE);

                        // second table walk
                        let PTE_addr = (PTE >> 10 & 0x3FFFFF) * PAGESIZE + VPN0 * PTESIZE;
                        let PTE = match self.check_pte_validity(dram.load32(PTE_addr)) {
                            Ok(pte) => pte,
                            Err(()) => {
                                return Err(()) // exception
                            },
                        };
                        let PPN0 = (PTE >> 10 & 0x3FF) as usize;
                        println!("PTE_addr(2): 0x{:x}", PTE_addr);
                        println!("PTE(2): 0x{:x}", PTE);

                        // check PTE to be leaf
                        if !self.check_leaf_pte(PTE) {
                            return Err(()) // exception
                        }

                        println!("raw address:{:x}\n\t=> transrated address:{:x}",
                                 addr, PPN1 << 22 | PPN0 << 12 | page_off);

                        // return transrated address
                        Ok(PPN1 << 22 | PPN0 << 12 | page_off)
                    },
                }
            },
            // return raw address if privileged level is Machine
            _ => Ok(addr),
        }
    }
}
