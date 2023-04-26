//! # DPC Wrapper
//!
//! This module wraps the Nintendo 64's DPC registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::{register_access, HARDWARE};

register_access!(0x0410_0000, Registers);

/// A zero-size wrapper around the Nintendo 64's DPC registers.
///
/// This structure must be acquired via the global [`HARDWARE`][crate::HARDWARE]
/// variable:
///
/// ```rust
/// # use nintendo64_pac::{HARDWARE, HardwareError};
/// # unsafe fn test() -> Result<(), HardwareError> {
/// let dpc = HARDWARE.dpc.take()?;
/// #
/// # dpc.status.set_freeze();
/// #
/// # assert!(HARDWARE.dpc.take().is_err());
/// # dpc.drop();
/// # assert!(HARDWARE.dpc.take().is_ok());
/// #
/// # Ok(())
/// # }
/// # assert!(unsafe { test() }.is_ok());
/// ```
///
/// Once a reference has been acquired, registers can be accessed:
///
/// ```rust
/// # use nintendo64_pac::{HARDWARE, HardwareError};
/// # unsafe fn test() -> Result<(), HardwareError> {
/// # let dpc = HARDWARE.dpc.take()?;
/// #
/// dpc.status.set_freeze();
/// #
/// # assert!(HARDWARE.dpc.take().is_err());
/// # dpc.drop();
/// # assert!(HARDWARE.dpc.take().is_ok());
/// #
/// # Ok(())
/// # }
/// # assert!(unsafe { test() }.is_ok());
/// ```
///
/// If needed, the reference can be given back to the global variable:
///
/// ```rust
/// # use nintendo64_pac::{HARDWARE, HardwareError};
/// # unsafe fn test() -> Result<(), HardwareError> {
/// # let dpc = HARDWARE.dpc.take()?;
/// #
/// # dpc.status.set_freeze();
/// #
/// # assert!(HARDWARE.dpc.take().is_err());
/// dpc.drop();
/// # assert!(HARDWARE.dpc.take().is_ok());
/// #
/// # Ok(())
/// # }
/// # assert!(unsafe { test() }.is_ok());
/// ```
#[non_exhaustive]
pub struct Dpc {
    pub buffer_busy: BufferBusy,
    pub dma_current: DmaCurrent,
    pub dma_start: DmaStart,
    pub pipe_busy: PipeBusy,
    pub tmem_load: TmemLoad,
    pub dma_end: DmaEnd,
    pub status: Status,
    pub clock: Clock,
}

impl Dpc {
    /// Returns ownership of the DPC registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.dpc.drop(self) }
    }
}

#[non_exhaustive]
pub struct DmaStart;

impl DmaStart {
    pub fn get(&self) -> u32 {
        registers().start.read(DpcStartReg::DMEM_START_ADDRESS)
    }

    pub fn set(&self, start_address: u32) {
        registers()
            .start
            .write(DpcStartReg::DMEM_START_ADDRESS.val(start_address))
    }
}

#[non_exhaustive]
pub struct DmaEnd;

impl DmaEnd {
    pub fn get(&self) -> u32 {
        registers().end.read(DpcEndReg::DMEM_END_ADDRESS)
    }

    pub fn set(&self, end_address: u32) {
        registers()
            .end
            .write(DpcEndReg::DMEM_END_ADDRESS.val(end_address))
    }
}

#[non_exhaustive]
pub struct DmaCurrent;

impl DmaCurrent {
    pub fn get(&self) -> u32 {
        registers()
            .current
            .read(DpcCurrentReg::DMEM_CURRENT_ADDRESS)
    }

    pub fn set(&self, current_address: u32) {
        registers()
            .current
            .write(DpcCurrentReg::DMEM_CURRENT_ADDRESS.val(current_address))
    }
}

#[non_exhaustive]
pub struct Status;

impl Status {
    pub fn get_xbus_dmem_dma(&self) -> bool {
        registers().status.is_set(DpcStatusReg::XBUS_DMEM_DMA)
    }

    pub fn get_freeze(&self) -> bool {
        registers().status.is_set(DpcStatusReg::FREEZE)
    }

    pub fn get_flush(&self) -> bool {
        registers().status.is_set(DpcStatusReg::FLUSH)
    }

    pub fn get_start_gclk(&self) -> bool {
        registers().status.is_set(DpcStatusReg::START_GLCK)
    }

    pub fn is_tmem_busy(&self) -> bool {
        registers().status.is_set(DpcStatusReg::TMEM_BUSY)
    }

    pub fn is_pipe_busy(&self) -> bool {
        registers().status.is_set(DpcStatusReg::PIPE_BUSY)
    }

    pub fn is_cmd_busy(&self) -> bool {
        registers().status.is_set(DpcStatusReg::CMD_BUSY)
    }

    pub fn is_cbuf_busy(&self) -> bool {
        registers().status.is_set(DpcStatusReg::CBUF_READY)
    }

    pub fn is_dma_busy(&self) -> bool {
        registers().status.is_set(DpcStatusReg::DMA_BUSY)
    }

    pub fn is_end_valid(&self) -> bool {
        registers().status.is_set(DpcStatusReg::END_VALID)
    }

    pub fn is_start_valid(&self) -> bool {
        registers().status.is_set(DpcStatusReg::START_VALID)
    }

    pub fn clear_xbus_dmem_dma(&self) {
        registers()
            .status
            .write(DpcStatusReg::CLEAR_XBUS_DMEM_DMA::SET)
    }

    pub fn set_xbus_dmem_dma(&self) {
        registers()
            .status
            .write(DpcStatusReg::SET_XBUS_DMEM_DMA::SET)
    }

    pub fn clear_freeze(&self) {
        registers().status.write(DpcStatusReg::CLEAR_FREEZE::SET)
    }

    pub fn set_freeze(&self) {
        registers().status.write(DpcStatusReg::SET_FREEZE::SET)
    }

    pub fn clear_flush(&self) {
        registers().status.write(DpcStatusReg::CLEAR_FLUSH::SET)
    }

    pub fn set_flush(&self) {
        registers().status.write(DpcStatusReg::SET_FLUSH::SET)
    }

    pub fn clear_tmem_counter(&self) {
        registers().status.write(DpcStatusReg::CLEAR_TMEM_CTR::SET)
    }

    pub fn clear_pipe_counter(&self) {
        registers().status.write(DpcStatusReg::CLEAR_PIPE_CTR::SET)
    }

    pub fn clear_cmd_counter(&self) {
        registers().status.write(DpcStatusReg::CLEAR_CMD_CTR::SET)
    }

    pub fn clear_clock_counter(&self) {
        registers().status.write(DpcStatusReg::CLEAR_CLOCK_CTR::SET)
    }
}

#[non_exhaustive]
pub struct Clock;

impl Clock {
    pub fn get(&self) -> u32 {
        registers().clock.read(DpcClockReg::CLOCK_COUNTER)
    }
}

#[non_exhaustive]
pub struct BufferBusy;

impl BufferBusy {
    pub fn get(&self) -> u32 {
        registers().buf_busy.read(DpcBufBusyReg::CLOCK_COUNTER)
    }
}

#[non_exhaustive]
pub struct PipeBusy;

impl PipeBusy {
    pub fn get(&self) -> u32 {
        registers().pipe_busy.read(DpcPipeBusyReg::CLOCK_COUNTER)
    }
}

#[non_exhaustive]
pub struct TmemLoad;

impl TmemLoad {
    pub fn get(&self) -> u32 {
        registers().tmem.read(DpcTmemReg::CLOCK_COUNTER)
    }
}

register_structs! {
    Registers {
        (0x0000 => pub start: ReadWrite<u32, DpcStartReg::Register>),
        (0x0004 => pub end: ReadWrite<u32, DpcEndReg::Register>),
        (0x0008 => pub current: ReadWrite<u32, DpcCurrentReg::Register>),
        (0x000C => pub status: ReadWrite<u32, DpcStatusReg::Register>),
        (0x0010 => pub clock: ReadOnly<u32, DpcClockReg::Register>),
        (0x0014 => pub buf_busy: ReadOnly<u32, DpcBufBusyReg::Register>),
        (0x0018 => pub pipe_busy: ReadOnly<u32, DpcPipeBusyReg::Register>),
        (0x001C => pub tmem: ReadOnly<u32, DpcTmemReg::Register>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    DpcStartReg [
        /// [23:0], read/write
        DMEM_START_ADDRESS  OFFSET(0)  NUMBITS(24) [],
    ],

    DpcEndReg [
        /// [23:0], read/write
        DMEM_END_ADDRESS  OFFSET(0)  NUMBITS(24) [],
    ],

    DpcCurrentReg [
        /// [23:0], read/write
        DMEM_CURRENT_ADDRESS  OFFSET(0)  NUMBITS(24) [],
    ],

    DpcStatusReg [
        /// [0], read only
        XBUS_DMEM_DMA       OFFSET(0)  NUMBITS(1)  [],

        /// [1], read only
        FREEZE              OFFSET(1)  NUMBITS(1)  [],

        /// [2], read only
        FLUSH               OFFSET(2)  NUMBITS(1)  [],

        /// [3], read only
        START_GLCK          OFFSET(3)  NUMBITS(1)  [],

        /// [4], read only
        TMEM_BUSY           OFFSET(4)  NUMBITS(1)  [],

        /// [5], read only
        PIPE_BUSY           OFFSET(5)  NUMBITS(1)  [],

        /// [6], read only
        CMD_BUSY            OFFSET(6)  NUMBITS(1)  [],

        /// [7], read only
        CBUF_READY          OFFSET(7)  NUMBITS(1)  [],

        /// [8], read only
        DMA_BUSY            OFFSET(8)  NUMBITS(1)  [],

        /// [9], read only
        END_VALID           OFFSET(9)  NUMBITS(1)  [],

        /// [10], read only
        START_VALID         OFFSET(10) NUMBITS(1)  [],

        /// [0], write only
        CLEAR_XBUS_DMEM_DMA OFFSET(0)  NUMBITS(1)  [],

        /// [1], write only
        SET_XBUS_DMEM_DMA   OFFSET(1)  NUMBITS(1)  [],

        /// [2], write only
        CLEAR_FREEZE        OFFSET(2)  NUMBITS(1)  [],

        /// [3], write only
        SET_FREEZE          OFFSET(3)  NUMBITS(1)  [],

        /// [4], write only
        CLEAR_FLUSH         OFFSET(4)  NUMBITS(1)  [],

        /// [5], write only
        SET_FLUSH           OFFSET(5)  NUMBITS(1)  [],

        /// [6], write only
        CLEAR_TMEM_CTR      OFFSET(6)  NUMBITS(1)  [],

        /// [7], write only
        CLEAR_PIPE_CTR      OFFSET(7)  NUMBITS(1)  [],

        /// [8], write only
        CLEAR_CMD_CTR       OFFSET(8)  NUMBITS(1)  [],

        /// [9], write only
        CLEAR_CLOCK_CTR     OFFSET(9)  NUMBITS(1)  [],
    ],

    DpcClockReg [
        /// [23:0], read only
        CLOCK_COUNTER       OFFSET(0)  NUMBITS(24) [],
    ],

    DpcBufBusyReg [
        /// [23:0], read only
        CLOCK_COUNTER       OFFSET(0)  NUMBITS(24) [],
    ],

    DpcPipeBusyReg [
        /// [23:0], read only
        CLOCK_COUNTER       OFFSET(0)  NUMBITS(24) [],
    ],

    DpcTmemReg [
        /// [23:0], read only
        CLOCK_COUNTER       OFFSET(0)  NUMBITS(24) [],
    ],
}
