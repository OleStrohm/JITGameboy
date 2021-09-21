#![allow(dead_code)]

use crate::jit::JitBuilder;

#[repr(u8)]
pub enum Reg {
    BC,
    DE,
    HL,
    SP,
    AF,
}

impl Reg {
    pub fn to_mov_op(self) -> u8 {
        match self {
            Reg::BC => 0xB8,
            _ => unimplemented!(),
        }
    }
}

#[repr(u8)]
pub enum Dest {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

impl Dest {
    pub fn to_mov_op(self) -> u8 {
        match self {
            Dest::B => 0xB7,
            Dest::C => 0xB3,
            _ => unimplemented!(),
        }
    }
}

pub enum Instruction {
    Nop,
    LdRN(Reg, u16),
    LdDN(Dest, u8),
    Log,
}

impl Instruction {
    pub fn into_asm(self, builder: &mut JitBuilder) {
        match self {
            Instruction::Nop => (),
            Instruction::LdRN(r, n) => builder.make_mov_u16(r, n),
            Instruction::LdDN(d, n) => builder.make_mov_u8(d, n),
            Instruction::Log => builder.log(),
        };
    }
}

