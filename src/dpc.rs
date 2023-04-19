//! # DP Command Wrapper
//!
//! This module wraps the Nintendo 64's DPC registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::Writeable,
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's DPC registers.
const DPC_REGS_BASE: usize = 0x0410_0000;

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

    /// Freeze the RDP.
    pub fn freeze_rdp(&self) -> &Self {
        self.registers().status.write(Status::FREEZE::SET);
        self
    }
}

register_structs! {
    DpcRegisters {
        (0x0000 => pub dma_start: ReadWrite<u32, DpcDmaAddress::Register>),
        (0x0004 => pub dma_end: ReadWrite<u32, DpcDmaAddress::Register>),
        (0x0008 => pub dma_current: ReadWrite<u32, DpcDmaAddress::Register>),
        (0x000C => pub status: ReadWrite<u32, Status::Register>),
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

    Status [
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
