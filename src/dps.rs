//! # DPS Wrapper
//!
//! This module wraps the Nintendo 64's DPC registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use crate::{register_access, HARDWARE};

register_access!(0x0420_0000, Registers);

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
pub struct Dps {
    /// Contains getters and setters for `DPS_BUFTEST_ADDR_REG`.
    pub buffer_test_address: BufferTestAddress,

    /// Contains getters and setters for `DPS_BUFTEST_DATA_REG`.
    pub buffer_test_data: BufferTestData,

    /// Contains getters and setters for `DPS_TEST_MODE_REG`.
    pub buffer_test_mode: BufferTestMode,

    /// Contains getters and setters for `DPS_TBIST_REG`.
    pub tmem_bist: TmemBist,
}

impl Dps {
    /// Returns ownership of the DPS registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.dps.drop(self) }
    }
}

/// A zero-size wrapper around `DPS_TBIST_REG`.
#[non_exhaustive]
pub struct TmemBist;

impl TmemBist {
    pub fn get_check(&self) -> bool {
        registers().tbist.is_set(DpsTbistReg::BIST_CHECK)
    }

    pub fn get_go(&self) -> bool {
        registers().tbist.is_set(DpsTbistReg::BIST_GO)
    }

    pub fn get_done(&self) -> bool {
        registers().tbist.is_set(DpsTbistReg::BIST_DONE)
    }

    pub fn get_fail(&self) -> u32 {
        registers().tbist.read(DpsTbistReg::BIST_FAIL)
    }

    pub fn set_check(&self) {
        registers().tbist.write(DpsTbistReg::BIST_CHECK::SET)
    }

    pub fn set_go(&self) {
        registers().tbist.write(DpsTbistReg::BIST_GO::SET)
    }

    pub fn set_clear(&self) {
        registers().tbist.write(DpsTbistReg::BIST_CLEAR::SET)
    }
}

/// A zero-size wrapper around `DPS_TEST_MODE_REG`.
#[non_exhaustive]
pub struct BufferTestMode;

impl BufferTestMode {
    pub fn enable(&self) {
        registers().test_mode.write(DpsTestModeReg::MODE::SET)
    }

    pub fn disable(&self) {
        registers().test_mode.write(DpsTestModeReg::MODE::CLEAR)
    }
}

/// A zero-size wrapper around `DPS_BUFTEST_ADDR_REG`.
#[non_exhaustive]
pub struct BufferTestAddress;

impl BufferTestAddress {
    pub fn get(&self) -> u32 {
        registers().buf_test_addr.read(DpsBufTestAddrReg::ADDR)
    }

    pub fn set(&self, address: u32) {
        registers()
            .buf_test_addr
            .write(DpsBufTestAddrReg::ADDR.val(address))
    }
}

/// A zero-size wrapper around `DPS_BUFTEST_DATA_REG`.
#[non_exhaustive]
pub struct BufferTestData;

impl BufferTestData {
    pub fn get(&self) -> u32 {
        registers().buf_test_data.read(DpsBufTestDataReg::DATA)
    }

    pub fn set(&self, data: u32) {
        registers()
            .buf_test_data
            .write(DpsBufTestDataReg::DATA.val(data))
    }
}

register_structs! {
    Registers {
        (0x0000 => pub tbist: ReadWrite<u32, DpsTbistReg::Register>),
        (0x0004 => pub test_mode: ReadWrite<u32, DpsTestModeReg::Register>),
        (0x0008 => pub buf_test_addr: ReadWrite<u32, DpsBufTestAddrReg::Register>),
        (0x000C => pub buf_test_data: ReadWrite<u32, DpsBufTestDataReg::Register>),
        (0x0010 => @END),
    }
}

register_bitfields! {
    u32,

    DpsTbistReg [
        BIST_CHECK OFFSET(0) NUMBITS(1)  [],
        BIST_GO    OFFSET(1) NUMBITS(1)  [],
        BIST_DONE  OFFSET(2) NUMBITS(1)  [],
        BIST_FAIL  OFFSET(3) NUMBITS(8)  [],
        BIST_CLEAR OFFSET(2) NUMBITS(1)  [],
    ],

    DpsTestModeReg [
        MODE       OFFSET(0) NUMBITS(1)  [],
    ],

    DpsBufTestAddrReg [
        ADDR       OFFSET(0) NUMBITS(7)  [],
    ],

    DpsBufTestDataReg [
        DATA       OFFSET(0) NUMBITS(32) [],
    ]
}
