use crate::cpu::instruction::reg2str;
use crate::cpu::CrossIsaUtil;
use crate::{log, Isa};
use std::rc::Rc;

pub struct Register {
    regs: [u64; 32],
    isa: Rc<Isa>,
}

impl Register {
    pub fn new(isa: Rc<Isa>) -> Self {
        Register { regs: [0; 32], isa }
    }

    pub fn show(&self) {
        log::debugln!("=========================================== dump ============================================");
        for (num, reg) in self.regs.iter().enumerate() {
            match *self.isa {
                Isa::Rv32 => {
                    log::debug!("{:>4}: 0x{:08x}\t", reg2str(num), reg);
                    if (num + 1) % 4 == 0 {
                        log::debugln!("")
                    }
                }
                Isa::Rv64 => {
                    log::debug!("{:>4}: 0x{:016x}\t", reg2str(num), reg);
                    if (num + 1) % 3 == 0 {
                        log::debugln!("")
                    }
                }
            }
        }
        log::debugln!("\n=============================================================================================");
    }

    pub fn read(&self, src: Option<usize>) -> u64 {
        let src = src.unwrap();
        if src == 0 {
            0
        } else {
            self.regs[src].fix2regsz(&self.isa)
        }
    }

    pub fn write(&mut self, dist: Option<usize>, src: u64) {
        let dist = dist.unwrap();
        if dist != 0 {
            self.regs[dist] = src.fix2regsz(&self.isa);
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Self::new(Isa::Rv64.into())
    }
}
