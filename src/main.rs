mod jit;
mod mem;
mod op;

use std::slice::from_raw_parts_mut;

use jit::JitBuilder;
use op::Dest;
use op::Instruction;

use crate::op::Reg;

pub extern "C" fn jit_log(log: *mut u8) {
    let log = unsafe { from_raw_parts_mut(log, 8) };
    println!(" AF | BC | DE | HL |");
    println!(
        "{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|",
        log[1], log[0], log[3], log[2], log[5], log[4], log[7], log[6],
    );
}

fn main() {
    let mut builder = JitBuilder::new();
    Instruction::LdDN(Dest::A, 0x11).into_asm(&mut builder);
    Instruction::LdDN(Dest::B, 0x22).into_asm(&mut builder);
    Instruction::LdDN(Dest::C, 0x33).into_asm(&mut builder);
    Instruction::LdDN(Dest::D, 0x44).into_asm(&mut builder);
    Instruction::LdDN(Dest::E, 0x55).into_asm(&mut builder);
    Instruction::LdDN(Dest::H, 0x66).into_asm(&mut builder);
    Instruction::LdDN(Dest::L, 0x77).into_asm(&mut builder);
    Instruction::Log.into_asm(&mut builder);
    Instruction::LdDN(Dest::A, 0x0).into_asm(&mut builder);

    let mut log: [u8; 8] = [0; 8];

    let fun = builder.into_fn();
    println!("{:0X}", fun(log.as_mut_ptr(), jit_log));
    jit_log(log.as_mut_ptr());
}
