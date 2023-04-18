//! # DP Command Wrapper
//!
//! This module wraps the Nintendo 64's DPC registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's DPC registers.
const DPC_REGS_BASE: usize = 0x0410_0000;

/// A zero-size wrapper around the Nintendo 64's DPC registers.
///
/// This structure must be acquired via the global [`HARDWARE`][crate::HARDWARE]
/// variable:
///
/// ```rust
/// let dpc = unsafe { HARDWARE.dpc.take() }?;
/// ```
///
/// Once a reference has been acquired, registers can be accessed:
///
/// ```rust
/// let status = dpc.status();
/// let clock = dpc.clock();
///
/// dpc.set_dma_start(0x12345678)
///     .set_dma_end(0x12345678)
///     .set_dma_current(0x12345678);
/// ```
///
/// If needed, the reference can be given back to the global variable:
///
/// ```rust
/// dpc.drop();
/// ```
#[non_exhaustive]
pub struct Dpc;

impl Dpc {
    /// Gets a reference to the DPC registers.
    fn registers<'a>(&self) -> &'a DpcRegisters {
        unsafe { &mut *(DPC_REGS_BASE as *mut DpcRegisters) }
    }

    /// Returns ownership of the DPC registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.dpc.drop(self) }
    }

    /// Gets the starting DMA address.
    pub fn dma_start(&self) -> u32 {
        self.registers().dma_start.read(DpcDmaAddress::ADDRESS)
    }

    /// Sets the starting DMA address.
    pub fn set_dma_start(&self, dma_start: u32) -> &Self {
        self.registers()
            .dma_start
            .write(DpcDmaAddress::ADDRESS.val(dma_start));
        self
    }

    /// Gets the ending DMA address.
    pub fn dma_end(&self) -> u32 {
        self.registers().dma_end.read(DpcDmaAddress::ADDRESS)
    }

    /// Sets the ending DMA address.
    pub fn set_dma_end(&self, dma_end: u32) -> &Self {
        self.registers()
            .dma_end
            .write(DpcDmaAddress::ADDRESS.val(dma_end));
        self
    }

    /// Gets the current DMA address.
    pub fn dma_current(&self) -> u32 {
        self.registers().dma_current.read(DpcDmaAddress::ADDRESS)
    }

    /// Sets the current DMA address.
    pub fn set_dma_current(&self, dma_current: u32) -> &Self {
        self.registers()
            .dma_current
            .write(DpcDmaAddress::ADDRESS.val(dma_current));
        self
    }

    /// Gets the status.
    pub fn status(&self) -> u32 {
        todo!()
    }

    /// Sets the status.
    pub fn set_status(&self, _status: u32) -> &Self {
        todo!()
    }

    /// Gets the clock counter.
    pub fn clock(&self) -> u32 {
        self.registers().clock.read(DpcClockCounter::CLOCK_COUNTER)
    }

    /// Gets the BUFBUSY clock counter.
    pub fn buffer_busy(&self) -> u32 {
        self.registers()
            .buffer_busy
            .read(DpcClockCounter::CLOCK_COUNTER)
    }

    /// Gets the PIPEBUSY clock counter.
    pub fn pipe_busy(&self) -> u32 {
        self.registers()
            .pipe_busy
            .read(DpcClockCounter::CLOCK_COUNTER)
    }

    /// Gets the TMEM clock counter.
    pub fn texture_memory(&self) -> u32 {
        self.registers()
            .texture_memory
            .read(DpcClockCounter::CLOCK_COUNTER)
    }
}

register_structs! {
    DpcRegisters {
        (0x0000 => pub dma_start: ReadWrite<u32, DpcDmaAddress::Register>),
        (0x0004 => pub dma_end: ReadWrite<u32, DpcDmaAddress::Register>),
        (0x0008 => pub dma_current: ReadWrite<u32, DpcDmaAddress::Register>),
        (0x000C => pub status: ReadWrite<u32, DpcStatus::Register>),
        (0x0010 => pub clock: ReadOnly<u32, DpcClockCounter::Register>),
        (0x0014 => pub buffer_busy: ReadOnly<u32, DpcClockCounter::Register>),
        (0x0018 => pub pipe_busy: ReadOnly<u32, DpcClockCounter::Register>),
        (0x001C => pub texture_memory: ReadOnly<u32, DpcClockCounter::Register>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    DpcDmaAddress [
        ADDRESS       OFFSET(0)  NUMBITS(24) [],
    ],

    DpcStatus [
        XBUS_DMEM_DMA OFFSET(0)  NUMBITS(1)  [],
        FREEZE        OFFSET(1)  NUMBITS(1)  [],
        FLUSH         OFFSET(2)  NUMBITS(1)  [],
        START_GLCK    OFFSET(3)  NUMBITS(1)  [],
        TMEM_BUSY     OFFSET(4)  NUMBITS(1)  [],
        PIPE_BUSY     OFFSET(5)  NUMBITS(1)  [],
        CMD_BUSY      OFFSET(6)  NUMBITS(1)  [],
        CBUF_READY    OFFSET(7)  NUMBITS(1)  [],
        DMA_BUSY      OFFSET(8)  NUMBITS(1)  [],
        END_VALID     OFFSET(9)  NUMBITS(1)  [],
        START_VALID   OFFSET(10) NUMBITS(1)  [],
    ],

    DpcClockCounter [
        CLOCK_COUNTER OFFSET(0)  NUMBITS(24) [],
    ]
}
