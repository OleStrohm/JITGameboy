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

#[macro_export]
macro_rules! test {
    ($($op:expr $(;)?)* =>
     $(A: $a:literal $(,)?)?
     $(F: $f:literal $(,)?)?
     $(AF: $af:literal $(,)?)?
     $(B: $b:literal $(,)?)?
     $(C: $c:literal $(,)?)?
     $(BC: $bc:literal $(,)?)?
     $(D: $d:literal $(,)?)?
     $(E: $e:literal $(,)?)?
     $(DE: $de:literal $(,)?)?
     $(H: $h:literal $(,)?)?
     $(L: $l:literal $(,)?)?
     $(HL: $hl:literal $(,)?)?
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
        $(assert!($af == u16::from_le_bytes([log[0], log[1]]));)?
        $(assert!($b == log[3]);)?
        $(assert!($c == log[2]);)?
        $(assert!($bc == u16::from_le_bytes([log[2], log[3]]));)?
        $(assert!($d == log[5]);)?
        $(assert!($e == log[4]);)?
        $(assert!($de == u16::from_le_bytes([log[4], log[5]]));)?
        $(assert!($h == log[7]);)?
        $(assert!($l == log[6]);)?
        $(assert!($hl == u16::from_le_bytes([log[6], log[7]]));)?
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jit::JitBuilder;

    #[test]
    fn nop() {
        test! {
            Instruction::Nop;
            =>
            A: 0, F: 0, AF: 0,
            B: 0, C: 0, BC: 0,
            D: 0, E: 0, DE: 0,
            H: 0, L: 0, HL: 0,
        }
    }

    #[test]
    fn many_nop() {
        test! {
            Instruction::Nop;
            Instruction::Nop;
            Instruction::Nop;
            Instruction::Nop;
            Instruction::Nop;
            Instruction::Nop;
            Instruction::Nop;
            =>
            AF: 0, BC: 0,
            DE: 0, HL: 0,
        }
    }

    #[test]
    fn ld_dests() {
        //test! {
        //    Instruction::LdDN(Dest::A, 0x11);
        //    Instruction::LdDN(Dest::B, 0x22);
        //    Instruction::LdDN(Dest::C, 0x33);
        //    Instruction::LdDN(Dest::D, 0x44);
        //    Instruction::LdDN(Dest::E, 0x55);
        //    Instruction::LdDN(Dest::H, 0x66);
        //    Instruction::LdDN(Dest::L, 0x77);
        //    =>
        //    A: 0x11
        //    BC: 0x2233,
        //    DE: 0x4455,
        //    HL: 0x6677,
        //}
        extern "C" fn stub(_: *mut u8) {}
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
        println!("{:0X}", fun(log.as_mut_ptr(), stub));
    }
}
