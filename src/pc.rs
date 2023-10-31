//! # Program Counter

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// Program Counter Interface
pub struct Pc;

impl_interface!(Pc, PcRegisters, 0x0408_0000);

/// # Program Counter Register Block
#[repr(C)]
pub struct PcRegisters {
    /// `0x00` - Program counter
    pub sp_pc_reg: SpPcReg,

    /// `0x04` - IMEM BIST
    pub sp_ibist_reg: SpIbistReg,
}

bitfield! {
    /// # Stack Pointer Program Counter Register
    pub struct SpPcReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub program_counter: u16 [get ProgramCounter, try_set ProgramCounter] @ 0..12,
    }
}

bitfield! {
    /// # Stack Pointer IMEM BIST Register
    pub struct SpIbistReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub bist_check: bool @ 0,
        pub bist_go: bool @ 1,
        pub bist_done: bool [read_only] @ 2,
        pub bist_fail: u8 [read_only, get BistFail] @ 3..7,
        pub bist_clear: bool [write_only] @ 2,
    }
}

/// # Program Counter
#[derive(Debug)]
pub struct ProgramCounter(pub u16);

impl_deref!(ProgramCounter, u16);
impl_get!(ProgramCounter, u16);
impl_set!(ProgramCounter, u16, 0..12);

/// # BIST Failure
#[derive(Debug)]
pub struct BistFail(pub u8);

impl_deref!(BistFail, u8);
impl_get!(BistFail, u8);
