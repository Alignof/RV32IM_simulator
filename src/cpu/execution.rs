mod exe_inst_16;
mod exe_inst_32;

use memmap::Mmap;
use super::CPU;
use super::instruction::Instruction;
use crate::bus::dram::Dram;
use exe_inst_16::exe_cinst;
use exe_inst_32::exe_inst;

pub trait Execution {
    fn execution(&self, cpu: &mut CPU);
}

impl Execution for Instruction {
    fn execution(&self, cpu: &mut CPU) {
        if self.is_compressed {
            exe_cinst(self, cpu);
        } else {
            exe_inst(self, cpu);
        }
    }
}
