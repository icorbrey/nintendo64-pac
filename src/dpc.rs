//! # DP Command

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # DP Command
pub struct Dpc;

impl_interface!(Dpc, DpcRegisters, 0x0410_0000);

/// DP command register block.
#[repr(C)]
pub struct DpcRegisters {
    /// `0x00` - Start address
    pub dpc_start_reg: DpcStartReg,

    /// `0x04` - End address
    pub dpc_end_reg: DpcEndReg,

    /// `0x08` - Current address
    pub dpc_current_reg: DpcCurrentReg,

    /// `0x0C` - Status
    pub dpc_status_reg: DpcStatusReg,

    /// `0x10` - Clock
    pub dpc_clock_reg: DpcClockReg,

    /// `0x14` - Bufbusy
    pub dpc_bufbusy_reg: DpcBufbusyReg,

    /// `0x18` - Pipebusy
    pub dpc_pipebusy_reg: DpcPipebusyReg,

    /// `0x1C` - TMEM
    pub dpc_tmem_reg: DpcTmemReg,
}

bitfield! {
    /// # DP Command Start Register
    pub struct DpcStartReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub start_address: u32 [get RdramAddress, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # DP Command End Register
    pub struct DpcEndReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub end_address: u32 [get RdramAddress, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # DP Command Current Register
    pub struct DpcCurrentReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub current_address: u32 [read_only, get RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # DP Command Status Register
    pub struct DpcStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,

        pub xbus_dmem_dma: bool [read_only] @ 0,
        pub freeze: bool [read_only] @ 1,
        pub flush: bool [read_only] @ 2,
        pub start_gclk: bool [read_only] @ 3,
        pub tmem_busy: bool [read_only] @ 4,
        pub pipe_busy: bool [read_only] @ 5,
        pub cmd_busy: bool [read_only] @ 6,
        pub cbuf_ready: bool [read_only] @ 7,
        pub dma_busy: bool [read_only] @ 8,
        pub end_valid: bool [read_only] @ 9,
        pub start_valid: bool [read_only] @ 10,

        pub clear_xbus_dmem_dma: bool [write_only] @ 0,
        pub set_xbus_dmem_dma: bool [write_only] @ 1,
        pub clear_freeze: bool [write_only] @ 2,
        pub set_freeze: bool [write_only] @ 3,
        pub clear_flush: bool [write_only] @ 4,
        pub set_flush: bool [write_only] @ 5,
        pub clear_tmem_ctr: bool [write_only] @ 6,
        pub clear_pipe_ctr: bool [write_only] @ 7,
        pub clear_cmd_ctr: bool [write_only] @ 8,
        pub clear_clock_ctr: bool [write_only] @ 9,
    }
}

bitfield! {
    /// # DP Command Clock Register
    pub struct DpcClockReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clock_counter: u32 [read_only, get ClockCounter] @ 0..24,
    }
}

bitfield! {
    /// # DP Command Bufbusy Register
    pub struct DpcBufbusyReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clock_counter: u32 [read_only, get ClockCounter] @ 0..24,
    }
}

bitfield! {
    /// # DP Command Pipebusy Register
    pub struct DpcPipebusyReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clock_counter: u32 [read_only, get ClockCounter] @ 0..24,
    }
}

bitfield! {
    /// # DP Command TMEM Register
    pub struct DpcTmemReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clock_counter: u32 [read_only, get ClockCounter] @ 0..24,
    }
}

/// # RDRAM Address
#[derive(Debug)]
pub struct RdramAddress(pub u32);

impl_deref!(RdramAddress, u32);
impl_get!(RdramAddress, u32);
impl_set!(RdramAddress, u32, 0..24);

/// # Clock Counter
#[derive(Debug)]
pub struct ClockCounter(pub u32);

impl_deref!(ClockCounter, u32);
impl_get!(ClockCounter, u32);
