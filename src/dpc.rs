//! # Display Processor Command
//!
//! The Reality Display Processor control interface is found at `0x0410_0000`.
//!
//! The DP registers allow commands to be sent to the RDP and for RDP settings to be toggled. The
//! RDP works in tandem with the Video Interface to allow for hardware-­drawn polygons. The RDP
//! contains its own texture memory (4KB) from which it can texture rectangles or triangles. The
//! RDP also has the capability of converting textures from other color modes as well as drawing
//! shaded and z-­buffered triangles.
//!
//! Under normal game operation, the RDP is usually controlled via the RSP as the drawing commands
//! are very low level. However, it is possible to control the RDP directly from a running program
//! on the N64 without coding a specialized program for the RSP. Sending a command to the RDP is
//! done in a similar fashion to sending a DMA request to one of the DMA controllers onboard the
//! N64.
//!
//! ## See also:
//!
//! - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed>

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # DPC base address
pub const DPC_BASE_ADDR: u32 = 0x0410_0000;

/// # DPC
///
/// This interface allows commands to be sent to the RDP and for RDP settings to be toggled.
pub struct Dpc;

impl_interface!(Dpc, DpcRegisters, DPC_BASE_ADDR);

/// # DPC register block
#[repr(C)]
pub struct DpcRegisters {
    /// Command start location.
    pub dpc_start_reg: DpcStartReg,

    /// Command end location.
    pub dpc_end_reg: DpcEndReg,

    /// Current command load location.
    pub dpc_current_reg: DpcCurrentReg,

    /// Status.
    pub dpc_status_reg: DpcStatusReg,

    /// Clock.
    pub dpc_clock_reg: DpcClockReg,

    /// Command buffer busy.
    pub dpc_bufbusy_reg: DpcBufbusyReg,

    /// Graphics pipe busy.
    pub dpc_pipebusy_reg: DpcPipebusyReg,

    /// TMEM.
    pub dpc_tmem_reg: DpcTmemReg,
}

bitfield! {
    /// # `DPC_START_REG` (Command start location)
    ///
    /// This register specifies the start location in [RDRAM][crate::rdram::Rdram] or RSP DMEM where
    /// an RDP command can be found.
    ///
    /// ## RDRAM vs. RSP DMEM modes
    ///
    /// The RDP will pull the command from either RDRAM or RSP DMEM, depending on the state of
    /// [`DPC_STATUS_REG::xbus_dmem_dma`][DpcStatusReg::xbus_dmem_dma].
    ///
    /// When in RDRAM mode, the RDP can handle any valid address. However, be careful as caching
    /// issues may arise with cached areas of RDRAM. When in RSP DMEM mode, the RDP expects the
    /// address to be in the range of `0x0000` to `0x0FFF`. This is identical to the valid data
    /// memory range with respect to the RSP.
    ///
    /// This register should be written to before [`DPC_END_REG`][DpcEndReg], as writing to that
    /// register begins the process of loading a command from memory and performing it. Commands
    /// should be issued individually from memory, not as a block of commands.
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_START_REG_.280x00.29>
    pub struct DpcStartReg(pub u32): Debug {
        pub raw: u32 @ ..,

        /// The start location in [RDRAM][crate::rdram::Rdram] or RSP DMEM where an RDP command can
        /// be found.
        pub start_address: u32 [get RdramAddress, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # `DPC_END_REG` (Command end location)
    ///
    /// This register specifies the end location in [RDRAM][crate::rdram::Rdram] or RSP DMEM where
    /// an RDP command can be found.
    ///
    /// ## RDRAM vs. RSP DMEM modes
    ///
    /// The RDP will pull the command from either RDRAM or RSP DMEM, depending on the state of
    /// [`DPC_STATUS_REG::xbus_dmem_dma`][DpcStatusReg::xbus_dmem_dma].
    ///
    /// When in RDRAM mode, the RDP can handle any valid address. However, be careful as caching
    /// issues may arise with cached areas of RDRAM. When in RSP DMEM mode, the RDP expects the
    /// address to be in the range of `0x000` to `0xFFF`. This is identical to the valid data memory
    /// range with respect to the RSP.
    ///
    /// This register should be written to after [`DPC_START_REG`][DpcStartReg], as writing to this
    /// register begins the process of loading a command from memory and performing it. Commands
    /// should be issued individually from memory, not as a block of commands.
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_END_REG_.280x04.29>
    pub struct DpcEndReg(pub u32): Debug {
        pub raw: u32 @ ..,

        /// The end location in [RDRAM][crate::rdram::Rdram] or RSP DMEM where an RDP command can be
        /// found.
        pub end_address: u32 [get RdramAddress, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # `DPC_CURRENT_REG` (Current command load location)
    ///
    /// This register reflects the current position of the RDP in loading commands from memory.
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_CURRENT_REG_.280x08.29>
    pub struct DpcCurrentReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub current_address: u32 [read_only, get RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # `DPC_STATUS_REG` (Status)
    ///
    /// This register allows arbitrary modes to be set on the RDP. Most notably, this is how one
    /// would tell the RDP from which processor to pull its data.
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_STATUS_REG_.280x0C.29>
    pub struct DpcStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,

        /// Represents whether the RDP is expecting commands to come from
        /// [RDRAM][crate::rdram::Rdram] or from RSP DMEM.
        pub xbus_dmem_dma: bool [read_only, get RdpCommandSource] @ 0,

        /// Represents whether freeze has been set.
        pub freeze: bool [read_only] @ 1,

        /// Represents whether flush has been set.
        pub flush: bool [read_only] @ 2,

        /// Purpose unknown.
        pub start_gclk: bool [read_only] @ 3,

        /// Represents whether RDP TMEM is currently in use.
        pub tmem_busy: bool [read_only] @ 4,

        /// Represents whether the graphics pipe is currently in use.
        pub pipe_busy: bool [read_only] @ 5,

        /// Represents whether the RDP is currently executing a command.
        pub cmd_busy: bool [read_only] @ 6,

        /// Represents whether the RDP command buffer is currently in use.
        pub cbuf_ready: bool [read_only] @ 7,

        /// Represents whether the RDP command buffer is currently in use.
        pub dma_busy: bool [read_only] @ 8,

        /// Functionality unknown. Typically `false`. Often used to tell if it is safe to issue the
        /// next RDP command. Ensure this flag (as well as [`start_valid`][Self::start_valid]) is
        /// cleared before issuing another command to ensure that the next command won't overwhelm
        /// the RDP command buffer and freeze the console.
        pub end_valid: bool [read_only] @ 9,

        /// Functionality unknown. Typically `false`. Often used to tell if it is safe to issue the
        /// next RDP command. Ensure this flag (as well as [`end_valid`][Self::end_valid]) is
        /// cleared before issuing another command to ensure that the next command won't overwhelm
        /// the RDP command buffer and freeze the console.
        pub start_valid: bool [read_only] @ 10,

        /// Writing to this field sets [`xbus_dmem_dma`][Self::xbus_dmem_dma] to
        /// [`RdpCommandSource::Rdram`], indicating that the RDP should pull commands from
        /// [RDRAM][crate::rdram::Rdram].
        pub clear_xbus_dmem_dma: bool [write_only] @ 0,

        /// Writing to this field sets [`xbus_dmem_dma`][Self::xbus_dmem_dma] to
        /// [`RdpCommandSource::RspDmem`], indicating that the RDP should pull commands from RSP
        /// DMEM.
        pub set_xbus_dmem_dma: bool [write_only] @ 1,

        /// Writing to this field sets [`freeze`][Self::freeze] to `false`.
        pub clear_freeze: bool [write_only] @ 2,

        /// Writing to this field sets [`freeze`][Self::freeze] to `true`.
        pub set_freeze: bool [write_only] @ 3,

        /// Writing to this field sets [`flush`][Self::flush] to `false`.
        pub clear_flush: bool [write_only] @ 4,

        /// Writing to this field sets [`flush`][Self::flush] to `true`.
        pub set_flush: bool [write_only] @ 5,

        /// Writing to this field resets [`DPC_TMEM_REG`][DpcTmemReg] to 0.
        pub clear_tmem_ctr: bool [write_only] @ 6,

        /// Writing to this field resets [`DPC_PIPEBUSY_REG`][DpcPipebusyReg] to 0.
        pub clear_pipe_ctr: bool [write_only] @ 7,

        /// Writing to this field resets [`DPC_BUFBUSY_REG`][DpcBufbusyReg] to 0.
        pub clear_cmd_ctr: bool [write_only] @ 8,

        /// Writing to this field resets [`DPC_CLOCK_REG`][DpcClockReg] to 0.
        pub clear_clock_ctr: bool [write_only] @ 9,
    }
}

bitfield! {
    /// # `DPC_CLOCK_REG`
    ///
    /// This register's functionality is unknown. It can be reset by writing to the
    /// `clear_clock_ctr` field in [`DPC_STATUS_REG`][DpcStatusReg].
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_CLOCK_REG_.280x10.29>
    pub struct DpcClockReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clock_counter: u32 [read_only, get ClockCounter] @ 0..24,
    }
}

bitfield! {
    /// # `DPC_BUFBUSY_REG`
    ///
    /// This register's functionality is unknown. It can be reset by writing to the `clear_cmd_ctr`
    /// field in [`DPC_STATUS_REG`][DpcStatusReg]. Counts up to a predetermined value.
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_BUFBUSY_REG_.280x14.29>
    pub struct DpcBufbusyReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clock_counter: u32 [read_only, get ClockCounter] @ 0..24,
    }
}

bitfield! {
    /// # `DPC_PIPEBUSY_REG`
    ///
    /// This register's functionality is unknown. It can be reset by writing to the `clear_pipe_ctr`
    /// field in [`DPC_STATUS_REG`][DpcStatusReg]. Counts up to a predetermined value.
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_PIPEBUSY_REG_.280x18.29>
    pub struct DpcPipebusyReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clock_counter: u32 [read_only, get ClockCounter] @ 0..24,
    }
}

bitfield! {
    /// # `DPC_TMEM_REG`
    ///
    /// This register's functionality is unknown. It can be reset by writing to the `clear_tmem_ctr`
    /// field in [`DPC_STATUS_REG`][DpcStatusReg]. Counts up to a predetermined value.
    ///
    /// ## See also
    ///
    /// - <http://en64.shoutwiki.com/wiki/DP_Registers_Detailed#DP_TMEM_REG_.280x1C.29>
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

/// # RDP Command Source
///
/// Represents the place from which the RDP is expecting to pull commands from.
#[derive(Debug)]
pub enum RdpCommandSource {
    /// Indicates that the RDP is expecting to pull commands from RSP DMEM.
    RspDmem,

    /// Indicates that the RDP is expecting to pull commands from [RDRAM][crate::rdram::Rdram].
    Rdram,
}

impl From<bool> for RdpCommandSource {
    fn from(value: bool) -> Self {
        if value {
            Self::RspDmem
        } else {
            Self::Rdram
        }
    }
}

/// # Clock Counter
#[derive(Debug)]
pub struct ClockCounter(pub u32);

impl_deref!(ClockCounter, u32);
impl_get!(ClockCounter, u32);
