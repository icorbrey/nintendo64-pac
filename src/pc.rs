//! # Program counter (PC)

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # PC base address.
pub const PC_BASE_ADDR: u32 = 0x0408_0000;

/// # Program counter (PC)
pub struct Pc;

impl_interface!(Pc, PcRegisters, PC_BASE_ADDR);

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
        pub program_counter: u16 [get ProgramCounter, try_set ProgramCounter] @ 0..12,
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

/// # Program counter
#[derive(Debug)]
pub struct ProgramCounter(pub u16);

impl_deref!(ProgramCounter, u16);
impl_get!(ProgramCounter, u16);
impl_set!(ProgramCounter, u16, 0..12);

/// # BIST failure
#[derive(Debug)]
pub struct BistFail(pub u8);

impl_deref!(BistFail, u8);
impl_get!(BistFail, u8);
