use crate::cpu::decode::DecodeUtil;
use crate::cpu::instruction::OpecodeKind;
use crate::cpu::TrapCause;

fn quadrant0(inst: u16, opmap: &u8) -> Result<OpecodeKind, (Option<u64>, TrapCause, String)> {
    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_ADDI4SPN),
        0b010 => Ok(OpecodeKind::OP_C_LW),
        0b110 => Ok(OpecodeKind::OP_C_SW),
        _ => Err((
            Some(u64::from(inst)),
            TrapCause::IllegalInst,
            format!("opecode decoding failed, {inst:b}"),
        )),
    }
}

fn quadrant1(inst: u16, opmap: &u8) -> Result<OpecodeKind, (Option<u64>, TrapCause, String)> {
    let sr_flag: u8 = inst.slice(11, 10) as u8;
    let lo_flag: u8 = inst.slice(6, 5) as u8;
    let mi_flag: u8 = inst.slice(11, 7) as u8;

    match opmap {
        0b000 => match mi_flag {
            0b00000 => Ok(OpecodeKind::OP_C_NOP),
            _ => Ok(OpecodeKind::OP_C_ADDI),
        },
        0b001 => Ok(OpecodeKind::OP_C_JAL),
        0b010 => Ok(OpecodeKind::OP_C_LI),
        0b011 => match mi_flag {
            0b00010 => Ok(OpecodeKind::OP_C_ADDI16SP),
            _ => Ok(OpecodeKind::OP_C_LUI),
        },
        0b100 => match sr_flag {
            0b00 => Ok(OpecodeKind::OP_C_SRLI),
            0b01 => Ok(OpecodeKind::OP_C_SRAI),
            0b10 => Ok(OpecodeKind::OP_C_ANDI),
            0b11 => match lo_flag {
                0b00 => Ok(OpecodeKind::OP_C_SUB),
                0b01 => Ok(OpecodeKind::OP_C_XOR),
                0b10 => Ok(OpecodeKind::OP_C_OR),
                0b11 => Ok(OpecodeKind::OP_C_AND),
                _ => Err((
                    Some(u64::from(inst)),
                    TrapCause::IllegalInst,
                    format!("opecode decoding failed, {inst:b}"),
                )),
            },
            _ => Err((
                Some(u64::from(inst)),
                TrapCause::IllegalInst,
                format!("opecode decoding failed, {inst:b}"),
            )),
        },
        0b101 => Ok(OpecodeKind::OP_C_J),
        0b110 => Ok(OpecodeKind::OP_C_BEQZ),
        0b111 => Ok(OpecodeKind::OP_C_BNEZ),
        _ => Err((
            Some(u64::from(inst)),
            TrapCause::IllegalInst,
            format!("opecode decoding failed, {inst:b}"),
        )),
    }
}

fn quadrant2(inst: u16, opmap: &u8) -> Result<OpecodeKind, (Option<u64>, TrapCause, String)> {
    let lo_flag: u8 = inst.slice(6, 2) as u8;
    let mi_flag: u8 = inst.slice(11, 7) as u8;
    let hi_flag: u8 = inst.slice(12, 12) as u8;

    match opmap {
        0b000 => Ok(OpecodeKind::OP_C_SLLI),
        0b010 => Ok(OpecodeKind::OP_C_LWSP),
        0b100 => match hi_flag {
            0b0 => match lo_flag {
                0b0 => Ok(OpecodeKind::OP_C_JR),
                _ => Ok(OpecodeKind::OP_C_MV),
            },
            0b1 => match mi_flag {
                0b0 => Ok(OpecodeKind::OP_C_EBREAK),
                _ => match lo_flag {
                    0b0 => Ok(OpecodeKind::OP_C_JALR),
                    _ => Ok(OpecodeKind::OP_C_ADD),
                },
            },
            _ => Err((
                Some(u64::from(inst)),
                TrapCause::IllegalInst,
                format!("opecode decoding failed, {inst:b}"),
            )),
        },
        0b110 => Ok(OpecodeKind::OP_C_SWSP),
        _ => Err((
            Some(u64::from(inst)),
            TrapCause::IllegalInst,
            format!("opecode decoding failed, {inst:b}"),
        )),
    }
}

pub fn parse_opecode(inst: u16) -> Result<OpecodeKind, (Option<u64>, TrapCause, String)> {
    let opmap: u8 = inst.slice(15, 13) as u8;
    let quadrant: u8 = inst.slice(1, 0) as u8;

    if inst == 0b0000000000000000 {
        return Err((
            Some(u64::from(inst)),
            TrapCause::IllegalInst,
            format!("opecode decoding failed, {inst:b}"),
        ));
    }

    match quadrant {
        0b00 => quadrant0(inst, &opmap),
        0b01 => quadrant1(inst, &opmap),
        0b10 => quadrant2(inst, &opmap),
        _ => Err((
            Some(u64::from(inst)),
            TrapCause::IllegalInst,
            format!("opecode decoding failed, {inst:b}"),
        )),
    }
}

pub fn parse_rd(
    inst: u16,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rd: usize = (inst.slice(4, 2) + 8) as usize;
    let q1_rd: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_wide_rd: usize = inst.slice(11, 7) as usize;
    let q2_rd: usize = inst.slice(11, 7) as usize;

    match opkind {
        // Quadrant 0
        OpecodeKind::OP_C_ADDI4SPN => Ok(Some(q0_rd)),
        OpecodeKind::OP_C_LW => Ok(Some(q0_rd)),
        // Quadrant 1
        OpecodeKind::OP_C_SRLI => Ok(Some(q1_rd)),
        OpecodeKind::OP_C_SRAI => Ok(Some(q1_rd)),
        OpecodeKind::OP_C_ANDI => Ok(Some(q1_rd)),
        OpecodeKind::OP_C_SUB => Ok(Some(q1_rd)),
        OpecodeKind::OP_C_XOR => Ok(Some(q1_rd)),
        OpecodeKind::OP_C_OR => Ok(Some(q1_rd)),
        OpecodeKind::OP_C_AND => Ok(Some(q1_rd)),
        OpecodeKind::OP_C_LI => Ok(Some(q1_wide_rd)),
        OpecodeKind::OP_C_LUI => Ok(Some(q1_wide_rd)),
        OpecodeKind::OP_C_ADDI => Ok(Some(q1_wide_rd)),
        // Quadrant 2
        OpecodeKind::OP_C_SLLI => Ok(Some(q2_rd)),
        OpecodeKind::OP_C_LWSP => Ok(Some(q2_rd)),
        OpecodeKind::OP_C_JR => Ok(Some(q2_rd)),
        OpecodeKind::OP_C_MV => Ok(Some(q2_rd)),
        OpecodeKind::OP_C_EBREAK => Ok(Some(q2_rd)),
        OpecodeKind::OP_C_JALR => Ok(Some(q2_rd)),
        OpecodeKind::OP_C_ADD => Ok(Some(q2_rd)),
        _ => Ok(None),
    }
}

pub fn parse_rs1(
    inst: u16,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rs1: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_rs1: usize = (inst.slice(9, 7) + 8) as usize;
    let q1_addi_rs1: usize = inst.slice(11, 7) as usize;
    let q2_rs1: usize = inst.slice(11, 7) as usize;

    match opkind {
        // Quadrant 0
        OpecodeKind::OP_C_LW => Ok(Some(q0_rs1)),
        OpecodeKind::OP_C_SW => Ok(Some(q0_rs1)),
        // Quadrant 1
        OpecodeKind::OP_C_ADDI => Ok(Some(q1_addi_rs1)),
        OpecodeKind::OP_C_ADDI16SP => Ok(Some(q1_addi_rs1)),
        OpecodeKind::OP_C_SRLI => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_SRAI => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_ANDI => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_SUB => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_XOR => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_OR => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_AND => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_BEQZ => Ok(Some(q1_rs1)),
        OpecodeKind::OP_C_BNEZ => Ok(Some(q1_rs1)),
        // Quadrant 2
        OpecodeKind::OP_C_SLLI => Ok(Some(q2_rs1)),
        OpecodeKind::OP_C_JR => Ok(Some(q2_rs1)),
        OpecodeKind::OP_C_JALR => Ok(Some(q2_rs1)),
        OpecodeKind::OP_C_ADD => Ok(Some(q2_rs1)),
        _ => Ok(None),
    }
}

pub fn parse_rs2(
    inst: u16,
    opkind: &OpecodeKind,
) -> Result<Option<usize>, (Option<u64>, TrapCause, String)> {
    // see riscv-spec-20191213.pdf, page 100, Table 16.2
    let q0_rs2: usize = (inst.slice(4, 2) + 8) as usize;
    let q1_rs2: usize = (inst.slice(4, 2) + 8) as usize;
    let q2_rs2: usize = inst.slice(6, 2) as usize;

    match opkind {
        // Quadrant 0
        OpecodeKind::OP_C_SW => Ok(Some(q0_rs2)),
        // Quadrant 1
        OpecodeKind::OP_C_SUB => Ok(Some(q1_rs2)),
        OpecodeKind::OP_C_XOR => Ok(Some(q1_rs2)),
        OpecodeKind::OP_C_OR => Ok(Some(q1_rs2)),
        OpecodeKind::OP_C_AND => Ok(Some(q1_rs2)),
        // Quadrant 2
        OpecodeKind::OP_C_MV => Ok(Some(q2_rs2)),
        OpecodeKind::OP_C_ADD => Ok(Some(q2_rs2)),
        OpecodeKind::OP_C_SWSP => Ok(Some(q2_rs2)),
        _ => Ok(None),
    }
}

pub fn parse_imm(
    inst: u16,
    opkind: &OpecodeKind,
) -> Result<Option<i32>, (Option<u64>, TrapCause, String)> {
    let q0_uimm = || (inst.slice(12, 10).set(&[5, 4, 3]) | inst.slice(6, 5).set(&[2, 6])) as i32;
    let q0_nzuimm = || inst.slice(12, 5).set(&[5, 4, 9, 8, 7, 6, 2, 3]) as i32;
    let q1_nzuimm =
        || (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
    let q1_nzimm = || {
        let imm16 = (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
        inst.to_signed_nbit(imm16, 6)
    };
    let q1_imm = || {
        let imm16 = (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
        inst.to_signed_nbit(imm16, 6)
    };
    let q1_j_imm = || {
        let imm16 = inst.slice(12, 2).set(&[11, 4, 9, 8, 10, 6, 7, 3, 2, 1, 5]) as i32;
        inst.to_signed_nbit(imm16, 12)
    };
    let q1_b_imm = || {
        let imm16 =
            (inst.slice(6, 2).set(&[7, 6, 2, 1, 5]) | inst.slice(12, 10).set(&[8, 4, 3])) as i32;
        inst.to_signed_nbit(imm16, 9)
    };
    let q1_16sp_nzimm = || {
        let imm16 = (inst.slice(6, 2).set(&[4, 6, 8, 7, 5]) | inst.slice(12, 12).set(&[9])) as i32;
        inst.to_signed_nbit(imm16, 10)
    };
    let q1_lui_imm = || {
        let imm16 =
            (inst.slice(6, 2).set(&[16, 15, 14, 13, 12]) | inst.slice(12, 12).set(&[17])) as i32;
        inst.to_signed_nbit(imm16, 18)
    };
    let q2_imm = || (inst.slice(6, 2).set(&[4, 3, 2, 1, 0]) | inst.slice(12, 12).set(&[5])) as i32;
    let q2_lwsp_imm =
        || (inst.slice(6, 2).set(&[4, 3, 2, 7, 6]) | inst.slice(12, 12).set(&[5])) as i32;
    let q2_swsp_imm = || inst.slice(12, 7).set(&[5, 4, 3, 2, 7, 6]) as i32;

    match opkind {
        // Quadrant0
        OpecodeKind::OP_C_ADDI4SPN => Ok(Some(q0_nzuimm())),
        OpecodeKind::OP_C_LW => Ok(Some(q0_uimm())),
        OpecodeKind::OP_C_SW => Ok(Some(q0_uimm())),
        // Quadrant1
        OpecodeKind::OP_C_NOP => Ok(Some(q1_nzimm())),
        OpecodeKind::OP_C_ADDI => Ok(Some(q1_nzimm())),
        OpecodeKind::OP_C_JAL => Ok(Some(q1_j_imm())),
        OpecodeKind::OP_C_LI => Ok(Some(q1_imm())),
        OpecodeKind::OP_C_ADDI16SP => Ok(Some(q1_16sp_nzimm())),
        OpecodeKind::OP_C_LUI => Ok(Some(q1_lui_imm())),
        OpecodeKind::OP_C_SRLI => Ok(Some(q1_nzuimm())),
        OpecodeKind::OP_C_SRAI => Ok(Some(q1_nzuimm())),
        OpecodeKind::OP_C_ANDI => Ok(Some(q1_imm())),
        OpecodeKind::OP_C_J => Ok(Some(q1_j_imm())),
        OpecodeKind::OP_C_BEQZ => Ok(Some(q1_b_imm())),
        OpecodeKind::OP_C_BNEZ => Ok(Some(q1_b_imm())),
        // Quadrant2
        OpecodeKind::OP_C_SLLI => Ok(Some(q2_imm())),
        OpecodeKind::OP_C_LWSP => Ok(Some(q2_lwsp_imm())),
        OpecodeKind::OP_C_SWSP => Ok(Some(q2_swsp_imm())),
        _ => Ok(None),
    }
}
