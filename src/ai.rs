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

register_access!(0x0450_0000, Registers);

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
/// # ai.control.enable_dma();
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
/// ai.control.enable_dma();
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
/// # ai.control.enable_dma();
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
    /// Returns ownership of the audio interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.audio_interface.drop(self) }
    }
}

/// A zero-size wrapper around `AI_DRAM_ADDR_REG`.
#[non_exhaustive]
pub struct DramAddr;

impl DramAddr {
    pub fn set(&self, dram_addr: u32) {
        registers()
            .dram_addr
            .write(AiDramAddrReg::STARTING_RDRAM_ADDRESS.val(dram_addr))
    }
}

/// A zero-size wrapper around `AI_LEN_REG`.
#[non_exhaustive]
pub struct Len;

impl Len {
    pub fn get_v1(&self) -> u32 {
        registers().len.read(AiLenReg::TRANSFER_LENGTH_V1)
    }

    pub fn set_v1(&self, len: u32) {
        registers().len.write(AiLenReg::TRANSFER_LENGTH_V1.val(len))
    }

    pub fn get_v2(&self) -> u32 {
        registers().len.read(AiLenReg::TRANSFER_LENGTH_V2)
    }

    pub fn set_v2(&self, len: u32) {
        registers().len.write(AiLenReg::TRANSFER_LENGTH_V2.val(len))
    }
}

/// A zero-size wrapper around `AI_CONTROL_REG`.
#[non_exhaustive]
pub struct Control;

impl Control {
    pub fn enable_dma(&self) {
        registers().control.write(AiControlReg::DMA_ENABLE::SET)
    }

    pub fn disable_dma(&self) {
        registers().control.write(AiControlReg::DMA_ENABLE::CLEAR)
    }
}

/// A zero-size wrapper around `AI_STATUS_REG`.
#[non_exhaustive]
pub struct Status;

impl Status {
    pub fn is_busy(&self) -> bool {
        registers().status.is_set(AiStatusReg::BUSY)
    }

    pub fn is_full(&self) -> bool {
        registers().status.is_set(AiStatusReg::FULL)
    }

    pub fn clear_interrupt(&self) {
        registers().status.write(AiStatusReg::CLEAR_INTERRUPT::SET)
    }
}

/// A zero-size wrapper around `AI_DACRATE_REG`.
#[non_exhaustive]
pub struct DacRate;

impl DacRate {
    pub fn set(&self, dac_rate: u32) {
        registers()
            .dac_rate
            .write(AiDacrateReg::DAC_RATE.val(dac_rate))
    }
}

/// A zero-size wrapper around `AI_BITRATE_REG`.
#[non_exhaustive]
pub struct BitRate;

impl BitRate {
    pub fn set(&self, bit_rate: u32) {
        registers()
            .bit_rate
            .write(AiBitrateReg::BIT_RATE.val(bit_rate))
    }
}

register_structs! {
    Registers {
        (0x0000 => pub dram_addr: WriteOnly<u32, AiDramAddrReg::Register>),
        (0x0004 => pub len: ReadWrite<u32, AiLenReg::Register>),
        (0x0008 => pub control: WriteOnly<u32, AiControlReg::Register>),
        (0x000C => pub status: ReadWrite<u32, AiStatusReg::Register>),
        (0x0010 => pub dac_rate: WriteOnly<u32, AiDacrateReg::Register>),
        (0x0014 => pub bit_rate: WriteOnly<u32, AiBitrateReg::Register>),
        (0x0018 => @END),
    }
}

register_bitfields! {
    u32,

    AiDramAddrReg [
        /// [23:0], write only
        STARTING_RDRAM_ADDRESS OFFSET(0)  NUMBITS(24) [],
    ],

    AiLenReg [
        /// [14:0], read/write
        TRANSFER_LENGTH_V1     OFFSET(0)  NUMBITS(15) [],

        /// [17:0], read/write
        TRANSFER_LENGTH_V2     OFFSET(0)  NUMBITS(18) [],
    ],

    AiControlReg [
        /// [0], write only
        DMA_ENABLE             OFFSET(0)  NUMBITS(1)  [],
    ],

    AiStatusReg [
        /// [0], write only
        CLEAR_INTERRUPT        OFFSET(0)  NUMBITS(1)  [],

        /// [30], read only
        BUSY                   OFFSET(30) NUMBITS(1)  [],

        /// [31], read only
        FULL                   OFFSET(31) NUMBITS(1)  [],
    ],

    AiDacrateReg [
        /// [13:0], write only
        DAC_RATE               OFFSET(0)  NUMBITS(14) [],
    ],

    AiBitrateReg [
        /// [3:0], write only
        BIT_RATE               OFFSET(0)  NUMBITS(4)  [],
    ],
}
