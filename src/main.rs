mod jit;
mod mem;
mod op;

use jit::JitBuilder;
use op::Instruction;
use op::Dest;

pub extern "C" fn jit_log(log: *mut u8) {
    println!(" AF | BC | DE | HL");
    println!(
        "{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|",
        unsafe { log.offset(1).read() },
        unsafe { log.offset(0).read() },
        unsafe { log.offset(3).read() },
        unsafe { log.offset(2).read() },
        unsafe { log.offset(5).read() },
        unsafe { log.offset(4).read() },
        unsafe { log.offset(7).read() },
        unsafe { log.offset(6).read() },
    );
}

fn main() {
    let mut builder = JitBuilder::new();
    Instruction::LdDN(Dest::B, 0x12).into_asm(&mut builder);
    Instruction::LdDN(Dest::C, 0x34).into_asm(&mut builder);
    Instruction::Log.into_asm(&mut builder);

    let mut log: [u8; 8] = [0; 8];

    let fun = builder.into_fn();
    println!("{:0X}", fun(log.as_mut_ptr(), jit_log));
}
