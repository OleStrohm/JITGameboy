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

pub extern "C" fn jit_log(_log: *mut u8) {
    //println!(" AF | BC | DE | HL");
    //println!(
    //    "{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|",
    //    unsafe { log.offset(1).read() },
    //    unsafe { log.offset(0).read() },
    //    unsafe { log.offset(3).read() },
    //    unsafe { log.offset(2).read() },
    //    unsafe { log.offset(5).read() },
    //    unsafe { log.offset(4).read() },
    //    unsafe { log.offset(7).read() },
    //    unsafe { log.offset(6).read() },
    //)

    //println!("hi");
    let a = unsafe { _log.read() };
}

fn main() {
    let mut builder = JitBuilder::new();
    Instruction::LdDN(Dest::B, 0x12).into_asm(&mut builder);
    Instruction::LdDN(Dest::C, 0x34).into_asm(&mut builder);
    Instruction::Log.into_asm(&mut builder);

    let mut log: [u8; 8] = [0; 8];

    let fun = builder.into_fn();
    println!("{:0X}", fun(log.as_mut_ptr(), jit_log));
    jit_log(log.as_mut_ptr());
}
