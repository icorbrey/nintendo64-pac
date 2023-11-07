//! # Program counter (PC)

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{fields, interface};

/// # PC base address.
pub const PC_BASE_ADDR: u32 = 0x0408_0000;

/// # Program counter (PC)
pub struct Pc;

interface!(Pc, PcRegisters, PC_BASE_ADDR);

/// # PC register block
#[repr(C)]
pub struct PcRegisters {
    /// Program counter
    pub sp_pc_reg: SpPcReg,

    /// IMEM BIST
    pub sp_ibist_reg: SpIbistReg,
}

bitfield! {
    /// # SP program counter register
    pub struct SpPcReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub program_counter: u16 [ProgramCounter] @ 0..12,
    }
}

bitfield! {
    /// # SP IMEM BIST register
    pub struct SpIbistReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub bist_check: bool @ 0,
        pub bist_go: bool @ 1,
        pub bist_done: bool [read_only] @ 2,
        pub bist_fail: u8 [read_only, get BistFail] @ 3..7,
        pub bist_clear: bool [write_only] @ 2,
    }
}

fields! [
    /// # BIST failure
    ux::u4 => BistFail,

    /// # Program counter
    ux::u12 => ProgramCounter,
];
