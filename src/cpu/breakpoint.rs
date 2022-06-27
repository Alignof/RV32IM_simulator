use crate::CPU;
use crate::cpu::{TransFor, TrapCause, PrivilegedLevel};
use crate::cpu::csr::CSRname;

impl CPU {
    pub fn check_breakpoint(&mut self, purpose: &TransFor, addr: u32) -> Result<u32, ()> {
        let tdata1 = self.csrs.read(CSRname::tdata1.wrap());
        let trigger_type = tdata1 >> 28 & 0xF;

        match trigger_type {
            0x0 => Ok(addr),
            0x1 => panic!("SiFive address match trigger is not implemented."),
            0x2 => {
                let tdata2 = self.csrs.read(CSRname::tdata2.wrap());
                let match_mode = tdata1 >> 7 & 0xF;
                let mode_m = tdata1 >> 6 & 0x1;
                let mode_s = tdata1 >> 4 & 0x1;
                let mode_u = tdata1 >> 3 & 0x1;
                
                if self.priv_lv == PrivilegedLevel::Machine && mode_m == 0x0 ||
                   self.priv_lv == PrivilegedLevel::Supervisor && mode_s == 0x0 || 
                   self.priv_lv == PrivilegedLevel::User && mode_u == 0x0 {
                       return Ok(addr);
                } 

                if match_mode != 0x0 {
                    panic!("this match mode is not supported");
                }

                match purpose {
                    TransFor::Fetch | TransFor::Deleg => {
                        if addr == tdata2 && tdata1 >> 2 & 0x1 == 1 {
                            self.exception(addr as i32, TrapCause::Breakpoint);
                            return Err(());
                        }
                        Ok(addr)
                    },
                    TransFor::Load => {
                        if addr == tdata2 && tdata1 & 0x1 == 1 {
                            self.exception(addr as i32, TrapCause::Breakpoint);
                            return Err(());
                        }
                        Ok(addr)
                    },
                    TransFor::StoreAMO => {
                        if addr == tdata2 && tdata1 >> 1 & 0x1 == 1 {
                            self.exception(addr as i32, TrapCause::Breakpoint);
                            return Err(());
                        }
                        Ok(addr)
                    },
                }
            },
            0x3 => panic!("Instruction count trigger is not implemented."),
            0x4 => panic!("Interrupt trigger is not implemented."),
            0x5 => panic!("Exception trigger is not implemented."),
            _ => panic!("this trigger is not supported: {}", trigger_type),
        }
    }
}
