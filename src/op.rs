#![allow(dead_code)]

use crate::jit::JitBuilder;

#[repr(u8)]
pub enum Reg {
    BC = 3,
    DE = 1,
    HL = 2,
    SP = 0xFE, // unused
    AF = 0xFF, // unused
}

#[repr(u8)]
pub enum Dest {
    B = 7,
    C = 3,
    D = 5,
    E = 1,
    H = 6,
    L = 2,
    HL = 0xFF, // unused
    A = 4,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jit::JitBuilder;

    macro_rules! test {
        ($($op:expr $(;)?)* =>
         $(A: $a:literal $(,)?)?
         $(F: $f:literal $(,)?)?
         $(B: $b:literal $(,)?)?
         $(C: $c:literal $(,)?)?
         $(D: $d:literal $(,)?)?
         $(E: $e:literal $(,)?)?
         $(H: $h:literal $(,)?)?
         $(L: $l:literal $(,)?)?
         ) => {
            extern "C" fn stub(_: *mut u8) {}
            let mut builder = JitBuilder::new();

            $(
                $op.into_asm(&mut builder);
            )*

            let mut log: [u8; 8] = [0; 8];
            let fun = builder.into_fn();
            fun(log.as_mut_ptr(), stub);
            $(assert!($a == log[1]);)?
            $(assert!($f == log[0]);)?
            $(assert!($b == log[3]);)?
            $(assert!($c == log[2]);)?
            $(assert!($d == log[5]);)?
            $(assert!($e == log[4]);)?
            $(assert!($h == log[7]);)?
            $(assert!($l == log[6]);)?
        };
    }

    #[test]
    fn nop() {
        test! {
            Instruction::Nop
            =>
            A: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
        }
    }

    #[test]
    fn many_nop() {
        test! {
            Instruction::Nop
            Instruction::Nop
            Instruction::Nop
            Instruction::Nop
            Instruction::Nop
            Instruction::Nop
            Instruction::Nop
            =>
            A: 0, F: 0,
            B: 0, C: 0,
            D: 0, E: 0,
            H: 0, L: 0,
        }
    }

    #[test]
    fn ld_regs_simple() {
        test! {
            Instruction::LdDN(Dest::A, 0x11)
            =>
            A: 0x11,
        }
    }
}
