mod inst_16;
mod inst_32;

use super::TrapCause;
use crate::cpu::instruction::{Extensions, Instruction, OpecodeKind};

pub trait Decode {
    fn decode(&self) -> Result<Instruction, (Option<u64>, TrapCause, String)>;
    fn parse_opecode(self) -> Result<OpecodeKind, &'static str>;
    fn parse_rd(
        self,
        opkind: &OpecodeKind,
    ) -> Result<Option<usize>, (Option<u64>, TrapCause, String)>;
    fn parse_rs1(
        self,
        opkind: &OpecodeKind,
    ) -> Result<Option<usize>, (Option<u64>, TrapCause, String)>;
    fn parse_rs2(
        self,
        opkind: &OpecodeKind,
    ) -> Result<Option<usize>, (Option<u64>, TrapCause, String)>;
    fn parse_imm(
        self,
        opkind: &OpecodeKind,
    ) -> Result<Option<i32>, (Option<u64>, TrapCause, String)>;
}

pub trait DecodeUtil {
    fn slice(self, end: u32, start: u32) -> Self;
    fn set(self, mask: &[u32]) -> Self;
    fn extension(self) -> Extensions;
    fn to_signed_nbit(&self, imm32: i32, bit_size: u32) -> i32 {
        let imm32 = imm32 & (2_i32.pow(bit_size) - 1);
        if imm32 >> (bit_size - 1) & 0x1 == 1 {
            imm32 - 2_i32.pow(bit_size)
        } else {
            imm32
        }
    }
}
