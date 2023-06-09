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
    /// Contains getters and setters for `DPC_BUFBUSY_REG`.
    pub buffer_busy: BufferBusy,

    /// Contains getters and setters for `DPC_CURRENT_REG`.
    pub dma_current: DmaCurrent,

    /// Contains getters and setters for `DPC_START_REG`.
    pub dma_start: DmaStart,

    /// Contains getters and setters for `DPC_PIPEBUSY_REG`.
    pub pipe_busy: PipeBusy,

    /// Contains getters and setters for `DPC_TMEM_REG`.
    pub tmem_load: TmemLoad,

    /// Contains getters and setters for `DPC_END_REG`.
    pub dma_end: DmaEnd,

    /// Contains getters and setters for `DPC_STATUS_REG`.
    pub status: Status,

    /// Contains getters and setters for `DPC_CLOCK_REG`.
    pub clock: Clock,
}

impl Dpc {
    /// Returns ownership of the DPC registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.dpc.drop(self) }
    }
}

/// A zero-size wrapper around `DPC_START_REG`.
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

/// A zero-size wrapper around `DPC_END_REG`.
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

/// A zero-size wrapper around `DPC_CURRENT_REG`.
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

/// A zero-size wrapper around `DPC_STATUS_REG`.
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

/// A zero-size wrapper around `DPC_CLOCK_REG`.
#[non_exhaustive]
pub struct Clock;

impl Clock {
    pub fn get(&self) -> u32 {
        registers().clock.read(DpcClockReg::CLOCK_COUNTER)
    }
}

/// A zero-size wrapper around `DPC_BUFBUSY_REG`.
#[non_exhaustive]
pub struct BufferBusy;

impl BufferBusy {
    pub fn get(&self) -> u32 {
        registers().buf_busy.read(DpcBufBusyReg::CLOCK_COUNTER)
    }
}

/// A zero-size wrapper around `DPC_PIPEBUSY_REG`.
#[non_exhaustive]
pub struct PipeBusy;

impl PipeBusy {
    pub fn get(&self) -> u32 {
        registers().pipe_busy.read(DpcPipeBusyReg::CLOCK_COUNTER)
    }
}

/// A zero-size wrapper around `DPC_TMEM_REG`.
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
        DMEM_START_ADDRESS    OFFSET(0)  NUMBITS(24) [],
    ],

    DpcEndReg [
        DMEM_END_ADDRESS      OFFSET(0)  NUMBITS(24) [],
    ],

    DpcCurrentReg [
        DMEM_CURRENT_ADDRESS  OFFSET(0)  NUMBITS(24) [],
    ],

    DpcStatusReg [
        XBUS_DMEM_DMA         OFFSET(0)  NUMBITS(1)  [],
        FREEZE                OFFSET(1)  NUMBITS(1)  [],
        FLUSH                 OFFSET(2)  NUMBITS(1)  [],
        START_GLCK            OFFSET(3)  NUMBITS(1)  [],
        TMEM_BUSY             OFFSET(4)  NUMBITS(1)  [],
        PIPE_BUSY             OFFSET(5)  NUMBITS(1)  [],
        CMD_BUSY              OFFSET(6)  NUMBITS(1)  [],
        CBUF_READY            OFFSET(7)  NUMBITS(1)  [],
        DMA_BUSY              OFFSET(8)  NUMBITS(1)  [],
        END_VALID             OFFSET(9)  NUMBITS(1)  [],
        START_VALID           OFFSET(10) NUMBITS(1)  [],

        CLEAR_XBUS_DMEM_DMA   OFFSET(0)  NUMBITS(1)  [],
        SET_XBUS_DMEM_DMA     OFFSET(1)  NUMBITS(1)  [],
        CLEAR_FREEZE          OFFSET(2)  NUMBITS(1)  [],
        SET_FREEZE            OFFSET(3)  NUMBITS(1)  [],
        CLEAR_FLUSH           OFFSET(4)  NUMBITS(1)  [],
        SET_FLUSH             OFFSET(5)  NUMBITS(1)  [],
        CLEAR_TMEM_CTR        OFFSET(6)  NUMBITS(1)  [],
        CLEAR_PIPE_CTR        OFFSET(7)  NUMBITS(1)  [],
        CLEAR_CMD_CTR         OFFSET(8)  NUMBITS(1)  [],
        CLEAR_CLOCK_CTR       OFFSET(9)  NUMBITS(1)  [],
    ],

    DpcClockReg [
        CLOCK_COUNTER         OFFSET(0)  NUMBITS(24) [],
    ],

    DpcBufBusyReg [
        CLOCK_COUNTER         OFFSET(0)  NUMBITS(24) [],
    ],

    DpcPipeBusyReg [
        CLOCK_COUNTER         OFFSET(0)  NUMBITS(24) [],
    ],

    DpcTmemReg [
        CLOCK_COUNTER         OFFSET(0)  NUMBITS(24) [],
    ],
}
