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
#[cfg(target_vendor = "nintendo64")]
const DPC_REGS_BASE: usize = 0x0410_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: DpcRegisters = unsafe { std::mem::zeroed() };
}

/// A zero-size wrapper around the Nintendo 64's audio interface registers.
///
/// This structure must be acquired via the global [`HARDWARE`][crate::HARDWARE]
/// variable:
///
/// ```rust
/// # use nintendo64_pac::{HARDWARE, HardwareError};
/// # unsafe fn test() -> Result<(), HardwareError> {
/// let dpc = HARDWARE.dpc.take()?;
/// #
/// # dpc.freeze_rdp();
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
/// dpc.freeze_rdp();
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
/// # dpc.freeze_rdp();
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
pub struct Dpc;

impl Dpc {
    /// Gets a reference to the DPC registers.
    #[cfg(target_vendor = "nintendo64")]
    fn registers<'a>(&self) -> &'a DpcRegisters {
        unsafe { &mut *(DPC_REGS_BASE as *mut DpcRegisters) }
    }

    /// Returns a reference to the audio interface registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
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

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for DpcRegisters {}

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
