use crate::cpu::CPU;
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_cinst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: u32 = 2;
    const REG_SP: usize = 2 as usize;
    const LINK_REG: usize = 1 as usize;

    match inst.opc {
        OP_C_ADDI4SPN => {
            cpu.reg[REG_SP] = 
                cpu.reg[REG_SP] + ((cpu.reg[inst.imm.unwrap() as usize] >> 2) & 0x1FF);
        },
        OP_C_FLD      => {},
        OP_C_LW       => {},
        OP_C_FLW      => {},
        OP_C_FSD      => {},
        OP_C_SW       => {},
        OP_C_FSW      => {},
        OP_C_NOP      => {/* NOP */},
        OP_C_ADDI     => {
            cpu.reg[inst.rd.unwrap() as usize] += inst.rs1.unwrap() as i32;
        },
        OP_C_JAL      => {
            cpu.reg[1] = (cpu.pc + INST_SIZE) as i32; 
            cpu.pc += inst.imm.unwrap() as u32;
        },
        OP_C_LI       => {},
        OP_C_ADDI16SP => {
            cpu.reg[REG_SP] = 
                cpu.reg[REG_SP] + ((cpu.reg[inst.imm.unwrap() as usize] >> 4) & 0x1FF);
        },
        OP_C_LUI      => {
            cpu.reg[inst.rd.unwrap() as usize] = (inst.imm.unwrap() << 12) as i32;
        },
        OP_C_SRLI     => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] >> inst.imm.unwrap() as i32;
        },
        OP_C_SRAI     => {
            cpu.reg[inst.rd.unwrap() as usize] =
                (cpu.reg[inst.rs1.unwrap() as usize] as i32) >> inst.imm.unwrap() as i32;
        },
        OP_C_ANDI     => {
            cpu.reg[inst.rd.unwrap() as usize] &= inst.rs1.unwrap() as i32;
        },
        OP_C_SUB      => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] - cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_C_XOR      => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] ^ cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_C_OR       => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] | cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_C_AND      => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] & cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_C_J        => {
            cpu.pc += inst.imm.unwrap() as u32;
        },
        OP_C_BEQZ     => {
            if cpu.reg[inst.rs1.unwrap() as usize] == 0 {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_C_BNEZ     => {
            if cpu.reg[inst.rs1.unwrap() as usize] != 0 {
                cpu.pc += inst.imm.unwrap() as u32;
            } 
        },
        OP_C_SLLI     => {},
        OP_C_FLDSP    => {},
        OP_C_LWSP     => {},
        OP_C_FLWSP    => {},
        OP_C_JR       => {
            cpu.pc += cpu.reg[inst.rs1.unwrap() as usize] as u32;
        },
        OP_C_MV       => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_C_EBREAK   => {},
        OP_C_JALR     => {
            cpu.reg[LINK_REG] = (cpu.pc + INST_SIZE) as i32; 
            cpu.pc += (cpu.reg[inst.rs1.unwrap() as usize]  + inst.imm.unwrap()) as u32;
        },
        OP_C_ADD      => {
            cpu.reg[inst.rd.unwrap() as usize] =
                cpu.reg[inst.rs1.unwrap() as usize] + cpu.reg[inst.rs2.unwrap() as usize];
        },
        OP_C_FSDSP    => {},
        OP_C_SWSP     => {},
        OP_C_FSWSP    => {},
        _             => panic!("not a compressed Instruction"),
    }

    cpu.pc += 2;
}
