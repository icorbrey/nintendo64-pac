//! # Audio Interface Wrapper
//!
//! This module wraps the Nintendo 64's AI registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

use crate::{register_access, HARDWARE};

register_access!(0x0450_0000, AudioInterfaceRegisters);

/// A zero-size wrapper around the Nintendo 64's audio interface registers.
///
/// This structure must be acquired via the global [`HARDWARE`][crate::HARDWARE]
/// variable:
///
/// ```rust
/// # use nintendo64_pac::{HARDWARE, HardwareError};
/// # unsafe fn test() -> Result<(), HardwareError> {
/// let ai = HARDWARE.audio_interface.take()?;
/// #
/// # let is_busy = ai.status.is_busy();
/// # let is_full = ai.status.is_full();
/// #
/// # ai.dram_addr.set(0x12345678);
/// # ai.len.set_v1(123);
/// # ai.dac_rate.set(0x12345678);
/// # ai.control.start();
/// #
/// # assert!(HARDWARE.audio_interface.take().is_err());
/// # ai.drop();
/// # assert!(HARDWARE.audio_interface.take().is_ok());
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
/// # let ai = HARDWARE.audio_interface.take()?;
/// #
/// let is_busy = ai.status.is_busy();
/// let is_full = ai.status.is_full();
///
/// ai.dram_addr.set(0x12345678);
/// ai.len.set_v1(123);
/// ai.dac_rate.set(0x12345678);
/// ai.control.start();
/// #
/// # assert!(HARDWARE.audio_interface.take().is_err());
/// # ai.drop();
/// # assert!(HARDWARE.audio_interface.take().is_ok());
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
/// # let ai = HARDWARE.audio_interface.take()?;
/// #
/// # let is_busy = ai.status.is_busy();
/// # let is_full = ai.status.is_full();
/// #
/// # ai.dram_addr.set(0x12345678);
/// # ai.len.set_v1(123);
/// # ai.dac_rate.set(0x12345678);
/// # ai.control.start();
/// #
/// # assert!(HARDWARE.audio_interface.take().is_err());
/// ai.drop();
/// # assert!(HARDWARE.audio_interface.take().is_ok());
/// #
/// # Ok(())
/// # }
/// # assert!(unsafe { test() }.is_ok());
/// ```
#[non_exhaustive]
pub struct AudioInterface {
    pub dram_addr: DramAddr,
    pub len: Len,
    pub control: Control,
    pub status: Status,
    pub dac_rate: DacRate,
    pub bit_rate: BitRate,
}

impl AudioInterface {
    pub const fn new() -> Self {
        Self {
            dram_addr: DramAddr,
            dac_rate: DacRate,
            bit_rate: BitRate,
            control: Control,
            status: Status,
            len: Len,
        }
    }

    /// Returns ownership of the audio interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.audio_interface.drop(self) }
    }
}

#[non_exhaustive]
pub struct DramAddr;

impl DramAddr {
    pub fn set(&self, dram_addr: u32) {
        registers()
            .dram_addr
            .write(DramAddrReg::DRAM_ADDR.val(dram_addr))
    }
}

#[non_exhaustive]
pub struct Len;

impl Len {
    pub fn get_v1(&self) -> u32 {
        registers().len.read(LenReg::LENGTH_V1)
    }

    pub fn set_v1(&self, len: u32) {
        registers().len.write(LenReg::LENGTH_V1.val(len))
    }

    pub fn get_v2(&self) -> u32 {
        registers().len.read(LenReg::LENGTH_V2)
    }

    pub fn set_v2(&self, len: u32) {
        registers().len.write(LenReg::LENGTH_V2.val(len))
    }
}

#[non_exhaustive]
pub struct Control;

impl Control {
    pub fn start(&self) {
        registers().control.write(ControlReg::START::SET)
    }
}

#[non_exhaustive]
pub struct Status;

impl Status {
    pub fn is_busy(&self) -> bool {
        registers().status.is_set(StatusReg::BUSY)
    }

    pub fn is_full(&self) -> bool {
        registers().status.is_set(StatusReg::FULL)
    }

    pub fn clear_interrupt(&self) {
        registers().status.write(StatusReg::CLEAR_INTERRUPT::SET)
    }
}

#[non_exhaustive]
pub struct DacRate;

impl DacRate {
    pub fn set(&self, dac_rate: u32) {
        registers()
            .dac_rate
            .write(DacRateReg::DAC_RATE.val(dac_rate))
    }
}

#[non_exhaustive]
pub struct BitRate;

impl BitRate {
    pub fn set(&self, bit_rate: u32) {
        registers()
            .bit_rate
            .write(BitRateReg::BIT_RATE.val(bit_rate))
    }
}

register_structs! {
    AudioInterfaceRegisters {
        (0x0000 => pub dram_addr: WriteOnly<u32, DramAddrReg::Register>),
        (0x0004 => pub len: ReadWrite<u32, LenReg::Register>),
        (0x0008 => pub control: WriteOnly<u32, ControlReg::Register>),
        (0x000C => pub status: ReadWrite<u32, StatusReg::Register>),
        (0x0010 => pub dac_rate: WriteOnly<u32, DacRateReg::Register>),
        (0x0014 => pub bit_rate: WriteOnly<u32, BitRateReg::Register>),
        (0x0018 => @END),
    }
}

register_bitfields! {
    u32,

    DramAddrReg [
        DRAM_ADDR       OFFSET(0)  NUMBITS(24) [],
    ],

    LenReg [
        LENGTH_V1       OFFSET(0)  NUMBITS(15) [],
        LENGTH_V2       OFFSET(0)  NUMBITS(18) [],
    ],

    ControlReg [
        START           OFFSET(0)  NUMBITS(1)  [],
    ],

    StatusReg [
        BUSY            OFFSET(30) NUMBITS(1)  [],
        FULL            OFFSET(31) NUMBITS(1)  [],

        CLEAR_INTERRUPT OFFSET(0)  NUMBITS(1)  [],
    ],

    DacRateReg [
        DAC_RATE        OFFSET(0)  NUMBITS(14) [],
    ],

    BitRateReg [
        BIT_RATE        OFFSET(0)  NUMBITS(4)  [],
    ],
}
