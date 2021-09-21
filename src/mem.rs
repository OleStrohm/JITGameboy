use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut},
};

const PAGE_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub enum MemError {
    AllocError,
}

pub struct JitMemory {
    size: usize,
    mem: *mut u8,
}

impl JitMemory {
    pub fn from_vec(buffer: Vec<u8>) -> Result<JitMemory, MemError> {
        let jit_mem = JitMemory::alloc(buffer.len() + 1)?;

        unsafe {
            std::ptr::copy_nonoverlapping(buffer.as_ptr(), jit_mem.mem, buffer.len());
        }

        Ok(jit_mem)
    }

    pub fn alloc(size: usize) -> Result<JitMemory, MemError> {
        let size = ((size + PAGE_SIZE - 1) / PAGE_SIZE) * PAGE_SIZE;

        let mut mem: *mut libc::c_void;

        unsafe {
            mem = MaybeUninit::uninit().assume_init();
            let permissions = libc::PROT_EXEC | libc::PROT_READ | libc::PROT_WRITE;
            if libc::posix_memalign(&mut mem, PAGE_SIZE, size) != 0 {
                return Err(MemError::AllocError);
            }
            if libc::mprotect(mem, size, permissions) != 0 {
                return Err(MemError::AllocError);
            }
            libc::memset(mem, 0xc3, size);
        }

        Ok(JitMemory {
            mem: mem as *mut u8,
            size,
        })
    }

    pub fn into_ptr(self) -> *mut u8 {
        self.mem
    }
}

impl Index<usize> for JitMemory {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        assert!(index < self.size);
        unsafe { &*self.mem.offset(index as isize) }
    }
}

impl IndexMut<usize> for JitMemory {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        assert!(index < self.size);
        unsafe { &mut *self.mem.offset(index as isize) }
    }
}
