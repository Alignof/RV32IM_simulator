use crate::cpu::{CPU, PrivilegedLevel};
use crate::cpu::csr::{CSRname, Mstatus};
use crate::cpu::instruction::{Instruction, OpecodeKind};

pub fn exe_inst(inst: &Instruction, cpu: &mut CPU) {
    use OpecodeKind::*;
    const INST_SIZE: usize = 4;

    // store previous program counter for excluding branch case
    let prev_pc = cpu.pc;

    match inst.opc {
        OP_LUI => {
            cpu.regs.write(inst.rd, inst.imm.unwrap() << 12);
        },
        OP_AUIPC => {
            cpu.regs.write(inst.rd, cpu.pc as i32 + (inst.imm.unwrap() << 12));
        },
        OP_JAL => {
            cpu.regs.write(inst.rd, (cpu.pc + INST_SIZE) as i32); 
            cpu.add2pc(inst.imm.unwrap());
        },
        OP_JALR => {
            let next_pc = cpu.pc + INST_SIZE;
            // setting the least-significant bit of the result to zero->vvvvvv
            cpu.update_pc((cpu.regs.read(inst.rs1)  + inst.imm.unwrap()) & !0x1);
            cpu.regs.write(inst.rd, next_pc as i32); 
        },
        OP_BEQ => {
            if cpu.regs.read(inst.rs1) == cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BNE => {
            if cpu.regs.read(inst.rs1) != cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BLT => {
            if cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BGE => {
            if cpu.regs.read(inst.rs1) >= cpu.regs.read(inst.rs2) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BLTU => {
            if (cpu.regs.read(inst.rs1) as u32) < (cpu.regs.read(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_BGEU => {
            if (cpu.regs.read(inst.rs1) as u32) >= (cpu.regs.read(inst.rs2) as u32) {
                cpu.add2pc(inst.imm.unwrap());
            } 
        },
        OP_LB => {
            cpu.regs.write(inst.rd,  
                cpu.bus.load8((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LH => {
            cpu.regs.write(inst.rd,  
                cpu.bus.load16((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LW => {
            cpu.regs.write(inst.rd,  
                cpu.bus.load32((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LBU => {
            cpu.regs.write(inst.rd,  
                cpu.bus.load_u8((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_LHU => {
            cpu.regs.write(inst.rd,  
                cpu.bus.load_u16((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize));
        },
        OP_SB => {
            cpu.bus.store8((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.regs.read(inst.rs2));
        },
        OP_SH => {
            cpu.bus.store16((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.regs.read(inst.rs2));
        },
        OP_SW => {
            cpu.bus.store32((cpu.regs.read(inst.rs1) + inst.imm.unwrap()) as usize,
                         cpu.regs.read(inst.rs2));
        },
        OP_ADDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) + inst.imm.unwrap());
        },
        OP_SLTI => {
            cpu.regs.write(inst.rd,  
                (cpu.regs.read(inst.rs1) < inst.imm.unwrap()) as i32);
        },
        OP_SLTIU => {
            cpu.regs.write(inst.rd,  
                ((cpu.regs.read(inst.rs1) as u32) < inst.imm.unwrap() as u32) as i32);
        },
        OP_XORI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) ^ inst.imm.unwrap());
        },
        OP_ORI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) | inst.imm.unwrap());
        },
        OP_ANDI => {
            cpu.regs.write(inst.rd, cpu.regs.read(inst.rs1) & inst.imm.unwrap());
        },
        OP_SLLI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) << inst.imm.unwrap()) as i32);
        },                                                
        OP_SRLI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) >> inst.imm.unwrap()) as i32);
        },
        OP_SRAI => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as i32) >> inst.imm.unwrap()) as i32);
        },
        OP_ADD => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) + cpu.regs.read(inst.rs2));
        },
        OP_SUB => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) - cpu.regs.read(inst.rs2));
        },
        OP_SLL => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) << cpu.regs.read(inst.rs2)) as i32);
        },
        OP_SLT => {
            cpu.regs.write(inst.rd,
                (cpu.regs.read(inst.rs1) < cpu.regs.read(inst.rs2)) as i32);
        },
        OP_SLTU => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32) < (cpu.regs.read(inst.rs2) as u32)) as i32);
        },
        OP_XOR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) ^ cpu.regs.read(inst.rs2));
        },
        OP_SRL => {
            cpu.regs.write(inst.rd,
                ((cpu.regs.read(inst.rs1) as u32)  >> cpu.regs.read(inst.rs2)) as i32);
        },
        OP_SRA => {
            cpu.regs.write(inst.rd,
                (cpu.regs.read(inst.rs1) as i32)  >> cpu.regs.read(inst.rs2));
        },
        OP_OR => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) | cpu.regs.read(inst.rs2));
        },
        OP_AND => {
            cpu.regs.write(inst.rd,
                cpu.regs.read(inst.rs1) & cpu.regs.read(inst.rs2));
        },
        OP_FENCE => {
            // nop (pipeline are not yet implemented)
        },
        OP_ECALL => {
            cpu.csrs.borrow_mut().write(CSRname::mcause.wrap(),
            match *(cpu.priv_lv.borrow()) {
                PrivilegedLevel::User => 8,
                PrivilegedLevel::Supervisor => 9,
                _ => panic!("cannot enviroment call in current privileged mode."),
            });
            cpu.csrs.borrow_mut().write(CSRname::mepc.wrap(), cpu.pc as i32);
            cpu.csrs.borrow_mut().bitclr(CSRname::mstatus.wrap(), 0x3 << 11);
            *(cpu.priv_lv.borrow_mut()) = PrivilegedLevel::Machine;
            let new_pc = cpu.csrs.borrow().read(CSRname::mtvec.wrap()) as i32;
            cpu.update_pc(new_pc);
        },
        OP_EBREAK => {
            panic!("not yet implemented: OP_EBREAK");
        },
        OP_CSRRW => {
            cpu.regs.write(inst.rd, cpu.csrs.borrow().read(inst.rs2) as i32);
            cpu.csrs.borrow_mut().write(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OP_CSRRS => {
            cpu.regs.write(inst.rd, cpu.csrs.borrow().read(inst.rs2) as i32);
            cpu.csrs.borrow_mut().bitset(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OP_CSRRC => {
            cpu.regs.write(inst.rd, cpu.csrs.borrow().read(inst.rs2) as i32);
            cpu.csrs.borrow_mut().bitclr(inst.rs2, cpu.regs.read(inst.rs1));
        },
        OP_CSRRWI => {
            cpu.regs.write(inst.rd, cpu.csrs.borrow().read(inst.rs2) as i32);
            cpu.csrs.borrow_mut().write(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_CSRRSI => {
            cpu.regs.write(inst.rd, cpu.csrs.borrow().read(inst.rs2) as i32);
            cpu.csrs.borrow_mut().bitset(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_CSRRCI => {
            cpu.regs.write(inst.rd, cpu.csrs.borrow().read(inst.rs2) as i32);
            cpu.csrs.borrow_mut().bitclr(inst.rs2, inst.rs1.unwrap() as i32);
        },
        OP_SRET => {
            let new_pc = cpu.csrs.borrow().read(CSRname::sepc.wrap()) as i32;
            cpu.update_pc(new_pc);
            *(cpu.priv_lv.borrow_mut()) = match cpu.csrs.borrow().read_mstatus(Mstatus::SPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b11 => panic!("invalid transition. (S-mode -> M-mode)"),
                _ => panic!("PrivilegedLevel 0x3 is Reserved."),
            }
        },
        OP_MRET => {
            let new_pc = cpu.csrs.borrow().read(CSRname::mepc.wrap()) as i32;
            cpu.update_pc(new_pc);
            *(cpu.priv_lv.borrow_mut()) = match cpu.csrs.borrow().read_mstatus(Mstatus::MPP) {
                0b00 => PrivilegedLevel::User,
                0b01 => PrivilegedLevel::Supervisor,
                0b11 => PrivilegedLevel::Machine,
                _ => panic!("PrivilegedLevel 0x3 is Reserved."),
            }
        },
        _ => panic!("not a full instruction"),
    }

    // add the program counter when it isn't a branch instruction
    if cpu.pc == prev_pc {
        cpu.add2pc(INST_SIZE as i32);
    }
}
