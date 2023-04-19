//! # DP Span Wrapper
//!
//! This module wraps the Nintendo 64's DPC registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

use crate::HARDWARE;

/// The static address of the Nintendo 64's DPS registers.
#[cfg(target_vendor = "nintendo64")]
const DPS_REGS_BASE: usize = 0x0420_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: DpsRegisters = unsafe { std::mem::zeroed() };
}

/// A zero-size wrapper around the Nintendo 64's DPC registers.
///
/// This structure must be acquired via the global [`HARDWARE`][crate::HARDWARE]
/// variable:
///
/// ```rust
/// # use nintendo64_pac::{HARDWARE, HardwareError};
/// # unsafe fn test() -> Result<(), HardwareError> {
/// let dps = HARDWARE.dps.take()?;
/// #
/// # assert!(HARDWARE.dps.take().is_err());
/// # dps.drop();
/// # assert!(HARDWARE.dps.take().is_ok());
/// #
/// # Ok(())
/// # }
/// # assert!(unsafe { test() }.is_ok());
/// ```
///
/// Once a reference has been acquired, registers can be accessed.
///
/// If needed, the reference can be given back to the global variable:
///
/// ```rust
/// # use nintendo64_pac::{HARDWARE, HardwareError};
/// # unsafe fn test() -> Result<(), HardwareError> {
/// # let dps = HARDWARE.dps.take()?;
/// #
/// # assert!(HARDWARE.dps.take().is_err());
/// dps.drop();
/// # assert!(HARDWARE.dps.take().is_ok());
/// #
/// # Ok(())
/// # }
/// # assert!(unsafe { test() }.is_ok());
/// ```
#[non_exhaustive]
pub struct Dps;

impl Dps {
    /// Gets a reference to the DPS registers.
    #[cfg(target_vendor = "nintendo64")]
    fn registers<'a>(&self) -> &'a DpsRegisters {
        unsafe { &mut *(DPS_REGS_BASE as *mut DpsRegisters) }
    }

    /// Returns a reference to the DPS registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the DPS registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.dps.drop(self) }
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for DpsRegisters {}

register_structs! {
    DpsRegisters {
        (0x0000 => pub texture_memory_bist: ReadWrite<u32, DpsTmemBist::Register>),
        (0x0004 => pub buffer_test_mode: ReadWrite<u32, DpsBufferTestMode::Register>),
        (0x0008 => pub buffer_test_address: ReadWrite<u32, DpsBufferTestAddress::Register>),
        (0x000C => pub buffer_test_data: ReadWrite<u32, DpsBufferTestData::Register>),
        (0x0010 => @END),
    }
}

register_bitfields! {
    u32,

    DpsTmemBist [
        BIST_CHECK OFFSET(0) NUMBITS(1)  [],
        BIST_GO    OFFSET(1) NUMBITS(1)  [],
        BIST_CLEAR OFFSET(2) NUMBITS(1)  [],
        BIST_DONE  OFFSET(2) NUMBITS(1)  [],
        BIST_FAIL  OFFSET(3) NUMBITS(8)  [],
    ],

    DpsBufferTestMode [
        TEST_MODE  OFFSET(0) NUMBITS(1)  [],
    ],

    DpsBufferTestAddress [
        TEST_ADDR  OFFSET(0) NUMBITS(7)  [],
    ],

    DpsBufferTestData [
        TEST_DATA  OFFSET(0) NUMBITS(32) [],
    ]
}
