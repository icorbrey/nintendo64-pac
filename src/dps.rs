//! # DP Span Wrapper
//!
//! This module wraps the Nintendo 64's DPC registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's DPS registers.
const DPS_REGS_BASE: usize = 0x0420_0000;

/// A zero-size wrapper around the Nintendo 64's audio interface registers.
///
/// This structure must be acquired via the global [`HARDWARE`][crate::HARDWARE]
/// variable:
///
/// ```rust
/// let dps = HARDWARE.audio_interface.take();
/// ```
///
/// Once a reference has been acquired, registers can be accessed:
///
/// ```rust
/// let bist = dps.texture_memory_bist();
///
/// dps.set_buffer_test_address(0x12345678)
///     .set_buffer_test_mode(123)
///     .set_buffer_test_data(12345678);
/// ```
///
/// If needed, the reference can be given back to the global variable:
///
/// ```rust
/// dps.drop();
/// ```
#[non_exhaustive]
pub struct Dps;

impl Dps {
    /// Gets a reference to the DPS registers.
    fn registers<'a>(&self) -> &'a DpsRegisters {
        unsafe { &mut *(DPS_REGS_BASE as *mut DpsRegisters) }
    }

    /// Returns ownership of the DPS registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.dps.drop(self) }
    }

    /// Get texture memory BIST data.
    pub fn texture_memory_bist(&self) -> u32 {
        todo!()
    }

    /// Set texture memory BIST data.
    pub fn set_texture_memory_bist(&self, _value: u32) -> &Self {
        todo!()
    }

    /// Get the buffer test mode.
    pub fn buffer_test_mode(&self) -> u32 {
        self.registers()
            .buffer_test_mode
            .read(DpsBufferTestMode::TEST_MODE)
    }

    /// Set the buffer test mode.
    pub fn set_buffer_test_mode(&self, mode: u32) -> &Self {
        self.registers()
            .buffer_test_mode
            .write(DpsBufferTestMode::TEST_MODE.val(mode));
        self
    }

    /// Get the buffer test address.
    pub fn buffer_test_address(&self) -> u32 {
        self.registers()
            .buffer_test_address
            .read(DpsBufferTestAddress::TEST_ADDR)
    }

    /// Set the buffer test address.
    pub fn set_buffer_test_address(&self, address: u32) -> &Self {
        self.registers()
            .buffer_test_address
            .write(DpsBufferTestAddress::TEST_ADDR.val(address));
        self
    }

    /// Get the buffer test data.
    pub fn buffer_test_data(&self) -> u32 {
        self.registers()
            .buffer_test_data
            .read(DpsBufferTestData::TEST_DATA)
    }

    /// Set the buffer test data.
    pub fn set_buffer_test_data(&self, data: u32) -> &Self {
        self.registers()
            .buffer_test_data
            .write(DpsBufferTestData::TEST_DATA.val(data));
        self
    }
}

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
