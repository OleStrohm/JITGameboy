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
    Output,
}

impl Instruction {
    pub fn into_asm(self, builder: &mut JitBuilder) {
        match self {
            Instruction::Nop => (),
            Instruction::LdRN(r, n) => builder.make_mov_u16(r, n),
            Instruction::LdDN(d, n) => builder.make_mov_u8(d, n),
            Instruction::Output => builder.output(),
        };
    }
}

fn main() {
    let mut builder = JitBuilder::new();
    Instruction::Nop.into_asm(&mut builder);
    //Instruction::LdDN(Dest::C, 0x12).into_asm(&mut builder);
    //Instruction::LdDN(Dest::B, 0x34).into_asm(&mut builder);

    let mut log_arr: [u8; 8] = [0; 8];

    let f = builder.into_fn();
    println!("{:0X}", f(0x1631));
    println!("{:?}", log_arr.as_mut_ptr())
}
