mod jit;
mod mem;
mod op;

use std::slice::from_raw_parts_mut;

use jit::JitBuilder;
use op::*;

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
    println!("{:0X}", fun(log.as_mut_ptr()));
}
