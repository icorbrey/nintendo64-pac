//! # DP Command

use core::ops::Deref;

use proc_bitfield::bitfield;

const DPC_BASE_REG: u32 = 0x0410_0000;

/// DP command.
pub struct Dpc;

impl Dpc {
    pub fn ptr() -> *const DpcRegisters {
        DPC_BASE_REG as *const _
    }
}

unsafe impl Sync for Dpc {}

impl Deref for Dpc {
    type Target = DpcRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// DP command register block.
#[repr(C)]
pub struct DpcRegisters {
    /// `0x00` - Start register #TK
    pub dpc_start_reg: DpcStartReg,

    /// `0x04` - End register #TK
    pub dpc_end_reg: DpcEndReg,

    /// `0x08` - Current register #TK
    pub dpc_current_reg: DpcCurrentReg,

    /// `0x0C` - Status register #TK
    pub dpc_status_reg: DpcStatusReg,

    /// `0x10` - Clock register #TK
    pub dpc_clock_reg: DpcClockReg,

    /// `0x14` - Bufbusy register #TK
    pub dpc_bufbusy_reg: DpcBufbusyReg,

    /// `0x18` - Pipebusy register #TK
    pub dpc_pipebusy_reg: DpcPipebusyReg,

    /// `0x1C` - TMEM register #TK
    pub dpc_tmem_reg: DpcTmemReg,
}

bitfield! {
    /// DP command start register.
    pub struct DpcStartReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub start_address: u32 @ 0..24,
    }
}

bitfield! {
    /// DP command end register.
    pub struct DpcEndReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub end_address: u32 @ 0..24,
    }
}

bitfield! {
    /// DP command current register.
    pub struct DpcCurrentReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub current_address: u32 @ 0..24
    }
}

bitfield! {
    /// DP command status register.
    pub struct DpcStatusReg(pub u32): Debug {
        /// Raw register access.
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
    /// DP command clock register.
    pub struct DpcClockReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub clock_counter: u32 [read_only] @ 0..24,
    }
}

bitfield! {
    /// DP command bufbusy register.
    pub struct DpcBufbusyReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub clock_counter: u32 [read_only] @ 0..24,
    }
}

bitfield! {
    /// DP command pipebusy register.
    pub struct DpcPipebusyReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub clock_counter: u32 [read_only] @ 0..24,
    }
}

bitfield! {
    /// DP command TMEM register.
    pub struct DpcTmemReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub clock_counter: u32 [read_only] @ 0..24,
    }
}
