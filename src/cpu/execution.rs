mod inst_16;
mod inst_32;

use crate::log;
use super::{CPU, TrapCause};
use super::instruction::{Instruction, Extensions};

pub trait Execution {
    fn execution(&self, cpu: &mut CPU) -> Result<(), (Option<u32>, TrapCause, String)>;
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut CPU) -> Result<(), (Option<u32>, TrapCause, String)>{
        log::debugln!("{:#?}", self);

        match self.opc_to_extension() {
            Extensions::C => inst_16::exe_cinst(self, cpu)?,
            _ => inst_32::exe_inst(self, cpu)?,
        }

        cpu.regs.show();
        Ok(())
    }
}
