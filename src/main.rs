mod mem;

use mem::JitMemory;

//enum Reg {
//    A, B, C, D, E, F, H, L,
//}

enum Instruction {
    Assign3,
}

impl Instruction {
    pub fn into_asm(self) -> impl Iterator<Item = u8> {
        IntoIterator::into_iter([0x48_u8, 0xc7, 0xc0, 0x03, 0x00, 0x00, 0x00])
    }
}

fn main() {
    let mut jit = JitMemory::alloc(7).unwrap();

    Instruction::Assign3
        .into_asm()
        .enumerate()
        .for_each(|(i, v)| jit[i] = v);

    let f = unsafe { jit.into_fn() };
    println!("{}", f());
}
