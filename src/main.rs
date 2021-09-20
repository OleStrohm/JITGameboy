mod jit;
mod mem;

use jit::JitBuilder;

#[allow(dead_code)]
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

#[allow(dead_code)]
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
            Dest::B => 0xB0,
            Dest::C => 0xB4,
            _ => unimplemented!(),
        }
    }
}

pub enum Instruction {
    Nop,
    LdRN(Reg, u16),
    LdDN(Dest, u8),
}

impl Instruction {
    pub fn into_asm(self, builder: &mut JitBuilder) {
        match self {
            Instruction::Nop => (),
            Instruction::LdRN(r, n) => builder.make_mov_u16(r, n),
            Instruction::LdDN(d, n) => builder.make_mov_u8(d, n),
        };
    }
}

fn main() {
    let mut builder = JitBuilder::new();
    Instruction::Nop.into_asm(&mut builder);
    Instruction::LdRN(Reg::BC, 10).into_asm(&mut builder);

    let f = builder.into_fn();
    println!("{}", f());
}
