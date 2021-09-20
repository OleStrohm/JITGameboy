use crate::mem::JitMemory;

use super::{Dest, Reg};

pub struct JitBuilder {
    buffer: Vec<u8>,
}

impl JitBuilder {
    pub fn new() -> JitBuilder {
        let mut buffer = Vec::new();

        // Zero out everything TODO: reload from previous frame
        buffer.extend([0x48, 0x31, 0xc0]); // xor rax, rax

        JitBuilder { buffer }
    }

    pub fn into_fn(self) -> fn() -> i64 {
        let mem = JitMemory::from_vec(self.buffer).unwrap();

        unsafe { mem.into_fn() }
    }

    pub fn make_mov_u8(&mut self, d: Dest, n: u8) {
        // mov r, n8
        self.buffer
            .extend([/*mov r*/ d.to_mov_op(), n]);
    }

    pub fn make_mov_u16(&mut self, r: Reg, n: u16) {
        let [n0, n1] = n.to_le_bytes();

        // mov r, n16
        self.buffer.extend([
            /*x86 mode?*/ 0x66,
            /*mov r*/ r.to_mov_op(),
            /*n*/ n0,
            n1,
        ]);
    }
}
