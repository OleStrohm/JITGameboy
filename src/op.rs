#![allow(dead_code)]

use std::slice::from_raw_parts_mut;

use crate::jit::JitBuilder;

pub extern "C" fn log(log: *mut u8) {
    let log = unsafe { from_raw_parts_mut(log, 8) };
    println!(" AF | BC | DE | HL |");
    println!(
        "{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|{:02X}{:02X}|",
        log[1], log[0], log[3], log[2], log[5], log[4], log[7], log[6],
    );
}

pub enum Op {
    Nop,
    LdRN(Reg, u16),
    AddHlR(Reg),
    IncR(Reg),
    DecR(Reg),
    IncD(Dest),
    DecD(Dest),
    LdDN(Dest, u8),
    Rotate(Dir, UseCarry),
    // JIT custom instructions
    Log,
}

impl Op {
    pub fn into_asm(self, builder: &mut JitBuilder) {
        match self {
            Op::Nop => (),
            Op::LdRN(r, n) => {
                let [n0, n1] = n.to_le_bytes();

                match r {
                    Reg::SP => builder.extend([0x66, 0x41, 0xb8, n0, n1]), // mov r8w, i16
                    Reg::AF => unimplemented!(),
                    r => {
                        // mov r, i16
                        builder.extend([0x66, /*mov r*/ 0xB8 + r as u8, /*n*/ n0, n1])
                    }
                }
            }
            Op::LdDN(d, n) => match d {
                Dest::HL => unimplemented!("Need to offset by memory location"),
                d => builder.extend([/*mov r*/ 0xb0 + d as u8, n]),
            },
            Op::AddHlR(r) => {
                let a: &[u8] = match r {
                    Reg::BC => &[0x66, 0x01, 0xda],       // add dx, bx
                    Reg::DE => &[0x66, 0x01, 0xca],       // add dx, cx
                    Reg::HL => &[0x66, 0x01, 0xd2],       // add dx, dx
                    Reg::SP => &[0x66, 0x44, 0x01, 0xc2], // add dx, r8w
                    _ => unreachable!(),
                };
                builder.extend(a.into_iter().copied());
            }
            Op::IncR(r) => {
                let a: &[u8] = match r {
                    Reg::BC => &[0x66, 0xff, 0xc3],       // inc bx
                    Reg::DE => &[0x66, 0xff, 0xc1],       // inc cx
                    Reg::HL => &[0x66, 0xff, 0xc2],       // inc dx
                    Reg::SP => &[0x66, 0x41, 0xff, 0xc0], // inc r8w
                    _ => unreachable!(),
                };
                builder.extend(a.into_iter().copied());
            }
            Op::DecR(r) => {
                let a: &[u8] = match r {
                    Reg::BC => &[0x66, 0xff, 0xcb],       // inc bx
                    Reg::DE => &[0x66, 0xff, 0xc9],       // inc cx
                    Reg::HL => &[0x66, 0xff, 0xca],       // inc dx
                    Reg::SP => &[0x66, 0x41, 0xff, 0xc8], // inc r8w
                    _ => unreachable!(),
                };
                builder.extend(a.into_iter().copied());
            }
            Op::IncD(d) => match d {
                Dest::HL => unimplemented!(),
                d => builder.extend([0xfe, 0xc0 + d as u8]), // inc <d>
            },
            Op::DecD(d) => match d {
                Dest::HL => unimplemented!(),
                d => builder.extend([0xfe, 0xc8 + d as u8]), // inc <d>
            },
            Op::Rotate(dir, c) => builder.extend([
                0xd0,
                match (dir, c) {
                    (Dir::Left, UseCarry::WithCarry) => 0xd4,
                    (Dir::Left, UseCarry::WithoutCarry) => 0xc4,
                    (Dir::Right, UseCarry::WithCarry) => 0xdc,
                    (Dir::Right, UseCarry::WithoutCarry) => 0xcc,
                },
            ]),
            Op::Log => builder.call_fn(log),
        };
    }
}

#[repr(u8)]
pub enum Dir {
    Left = 0,
    Right = 1,
}

#[repr(u8)]
pub enum UseCarry {
    WithCarry = 0,
    WithoutCarry = 1,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jit::JitBuilder;

    extern "C" fn stub(_: *mut u8) {}

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
            #[allow(unused_mut)]
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

    #[test]
    fn nop() {
        test! {
            Op::Nop;
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
            Op::Nop;
            Op::Nop;
            Op::Nop;
            Op::Nop;
            Op::Nop;
            Op::Nop;
            Op::Nop;
            =>
            AF: 0, BC: 0,
            DE: 0, HL: 0,
        }
    }

    #[test]
    fn ld_dests() {
        test! {
            Op::LdDN(Dest::A, 0x11);
            Op::LdDN(Dest::B, 0x22);
            Op::LdDN(Dest::C, 0x33);
            Op::LdDN(Dest::D, 0x44);
            Op::LdDN(Dest::E, 0x55);
            Op::LdDN(Dest::H, 0x66);
            Op::LdDN(Dest::L, 0x77);
            =>
            AF: 0x1100
            BC: 0x2233,
            DE: 0x4455,
            HL: 0x6677,
        }
    }

    #[test]
    fn ld_regs() {
        test! {
            Op::LdRN(Reg::BC, 0x1122);
            Op::LdRN(Reg::DE, 0x3344);
            Op::LdRN(Reg::HL, 0x5566);
            =>
            AF: 0, BC: 0x1122,
            DE: 0x3344, HL: 0x5566
        }
    }

    #[test]
    fn multiple_lds() {
        test! {
            Op::LdRN(Reg::BC, 0xBBCC);
            Op::LdRN(Reg::HL, 0x1234);
            Op::LdRN(Reg::BC, 0xAAAA);
            Op::LdDN(Dest::C, 0xFF);
            Op::LdDN(Dest::A, 0xAA);
            Op::LdDN(Dest::D, 0xF0);
            Op::LdRN(Reg::DE, 0x9876);
            Op::LdDN(Dest::E, 0xEE);
            =>
            A: 0xAA, BC: 0xAAFF,
            DE: 0x98EE, HL: 0x1234
        }
    }

    #[test]
    fn add_hl() {
        test! {
            Op::LdRN(Reg::BC, 0x1100);
            Op::LdRN(Reg::DE, 0x0011);
            Op::LdRN(Reg::HL, 0x2200);
            Op::LdRN(Reg::SP, 0x0033);
            Op::AddHlR(Reg::HL);
            Op::AddHlR(Reg::BC);
            Op::AddHlR(Reg::DE);
            Op::AddHlR(Reg::SP);
            =>
            HL: 0x5544
        }
    }

    #[test]
    fn inc() {
        test! {
            Op::IncR(Reg::BC);
            Op::IncR(Reg::DE);
            Op::IncR(Reg::HL);
            Op::IncD(Dest::A);
            Op::IncD(Dest::B);
            Op::IncD(Dest::C);
            Op::IncD(Dest::D);
            Op::IncD(Dest::E);
            Op::IncD(Dest::H);
            Op::IncD(Dest::L);
            =>
            A: 1, BC: 0x0102,
            DE: 0x0102, HL: 0x0102,
        }
    }

    #[test]
    fn dec() {
        test! {
            Op::LdDN(Dest::A, 0xFF);
            Op::LdRN(Reg::BC, 0xFFFF);
            Op::LdRN(Reg::DE, 0xFFFF);
            Op::LdRN(Reg::HL, 0xFFFF);
            Op::DecR(Reg::BC);
            Op::DecR(Reg::DE);
            Op::DecR(Reg::HL);
            Op::DecD(Dest::A);
            Op::DecD(Dest::B);
            Op::DecD(Dest::C);
            Op::DecD(Dest::D);
            Op::DecD(Dest::E);
            Op::DecD(Dest::H);
            Op::DecD(Dest::L);
            =>
            A: 0xFE, BC: 0xFEFD,
            DE: 0xFEFD, HL: 0xFEFD,
        }
    }

    #[test]
    fn rotate_single() {
        test! {
            Op::LdDN(Dest::A, 0xF0);
            Op::Rotate(Dir::Left, UseCarry::WithCarry);
            =>
            A: 0xE0,
        }
        test! {
            Op::LdDN(Dest::A, 0xF0);
            Op::Rotate(Dir::Left, UseCarry::WithoutCarry);
            =>
            A: 0xE1,
        }
        test! {
            Op::LdDN(Dest::A, 0x0F);
            Op::Rotate(Dir::Right, UseCarry::WithCarry);
            =>
            A: 0x07,
        }
        test! {
            Op::LdDN(Dest::A, 0x0F);
            Op::Rotate(Dir::Right, UseCarry::WithoutCarry);
            =>
            A: 0x87,
        }
    }
}
