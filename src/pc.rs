//! # Program Counter

use core::ops::Deref;

use proc_bitfield::bitfield;

const PC_BASE_REG: u32 = 0x0408_0000;

/// Program counter.
pub struct Pc;

impl Pc {
    pub fn ptr() -> *const PcRegisters {
        PC_BASE_REG as *const _
    }
}

unsafe impl Sync for Pc {}

impl Deref for Pc {
    type Target = PcRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Program counter register block.
#[repr(C)]
pub struct PcRegisters {
    /// 0x00 - Program counter
    pub sp_pc_reg: SpPcReg,

    /// 0x04 - IMEM BIST
    pub sp_ibist_reg: SpIbistReg,
}

bitfield! {
    /// Stack pointer program counter register.
    pub struct SpPcReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub program_counter: u16 @ 0..12,
    }
}

bitfield! {
    /// Stack pointer IMEM BIST register.
    pub struct SpIbistReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub bist_check: bool @ 0,
        pub bist_go: bool @ 1,

        pub bist_done: bool [read_only] @ 2,
        pub bist_fail: u8 [read_only] @ 3..7,

        pub bist_clear: bool [write_only] @ 2,
    }
}
