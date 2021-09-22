mod jit;
mod mem;
mod op;

use std::slice::from_raw_parts_mut;

use jit::JitBuilder;
use op::*;

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

    let ops = [
        Op::LdRN(Reg::BC, 0x1100),
        Op::LdRN(Reg::DE, 0x0011),
        Op::LdRN(Reg::HL, 0x2200),
        Op::LdRN(Reg::SP, 0x0033),
        Op::AddHlR(Reg::HL),
        Op::AddHlR(Reg::BC),
        Op::AddHlR(Reg::DE),
        Op::AddHlR(Reg::SP),
        Op::Log,
    ];
    for op in ops {
        op.into_asm(&mut builder);
    }

    let mut log: [u8; 8] = [0; 8];

    let fun = builder.into_fn();
    println!("{:0X}", fun(log.as_mut_ptr(), jit_log));
    jit_log(log.as_mut_ptr());
}
