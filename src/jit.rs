use std::mem;

use crate::mem::JitMemory;

use crate::op::{Dest, Reg};

pub struct JitBuilder {
    buffer: Vec<u8>,
}

impl JitBuilder {
    pub fn new() -> JitBuilder {
        let mut buffer = Vec::new();

        // Zero out everything
        // TODO: reload from previous frame
        buffer.extend([0x55]); // push rbp
        buffer.extend([0x53]); // push rbp
        buffer.extend([0x57]); // push rbp
        buffer.extend([0x56]); // push rbp
        buffer.extend([0x54]); // push rbp
        buffer.extend([0x49, 0x89, 0xfa]); // mov r10, rdi
        buffer.extend([0x48, 0x31, 0xc0]); // xor rax, rax
        buffer.extend([0x48, 0x31, 0xdb]); // xor rbx, rbx
        buffer.extend([0x48, 0x31, 0xc9]); // xor rcx, rcx
        buffer.extend([0x48, 0x31, 0xd2]); // xor rdx, rdx

        JitBuilder { buffer }
    }

    pub fn into_fn(mut self) -> fn(*mut u8, extern "C" fn(*mut u8)) -> i64 {
        // store registers
        self.buffer.extend([
            0x66, 0x41, 0x89, 0x02, // mov [r10], ax
            0x66, 0x41, 0x89, 0x5a, 0x02, // mov [r10+2], bx
            0x66, 0x41, 0x89, 0x4a, 0x04, // mov [r10+4], cx
            0x66, 0x41, 0x89, 0x52, 0x06, // mov [r10+6], dx
        ]);
        self.buffer.extend([0x48, 0x31, 0xc0]); // xor rax, rax
        self.buffer.extend([0x5c]); // pop rbp
        self.buffer.extend([0x5e]); // pop rbp
        self.buffer.extend([0x5f]); // pop rbp
        self.buffer.extend([0x5b]); // pop rbp
        self.buffer.extend([0x5d]); // pop rbp
        self.buffer.extend([0xc3]); // ret
        let mem = JitMemory::from_vec(self.buffer).unwrap();

        unsafe { mem::transmute(mem.into_ptr()) }
    }

    pub fn log(&mut self) {
        #[rustfmt::skip]
        self.buffer.extend([
            0x66, 0x41, 0x89, 0x02, // mov [r10], ax
            0x66, 0x41, 0x89, 0x5a, 0x02, // mov [r10+2], bx
            0x66, 0x41, 0x89, 0x4a, 0x04, // mov [r10+4], cx
            0x66, 0x41, 0x89, 0x52, 0x06, // mov [r10+6], dx
            0x4c, 0x89, 0xd7, // mov rdi, r10
            0x50, // push rbp
            0x53, // push rbp
            0x51, // push rbp
            0x52, // push rbp
            0x56, // push rbp
            0x41, 0x52, // push rbp
            0xff, 0xd6, // call rsi (jit_log)
            0x41, 0x5a, // push rbp
            0x5e, // pop rbp
            0x5a, // pop rbp
            0x59, // pop rbp
            0x5b, // pop rbp
            0x58, // pop rbp
        ]);
    }

    pub fn make_mov_u8(&mut self, d: Dest, n: u8) {
        match d {
            Dest::HL => unimplemented!("Need to offset by memory location"),
            d => self.buffer.extend([/*mov r*/ 0xb0 + d as u8, n]),
        }
    }

    pub fn make_mov_u16(&mut self, r: Reg, n: u16) {
        let [n0, n1] = n.to_le_bytes();

        // mov r, n16
        match r {
            Reg::SP => unimplemented!(),
            Reg::AF => unimplemented!(),
            r => self
                .buffer
                .extend([0x66, /*mov r*/ 0xB8 + r as u8, /*n*/ n0, n1]),
        }
    }
}
