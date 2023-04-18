//! # Audio Interface Wrapper
//!
//! This module wraps the Nintendo 64's AI registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's audio interface registers.
const AI_REGS_BASE: usize = 0x0450_0000;

/// A zero-size wrapper around the Nintendo 64's audio interface registers.
///
/// This structure must be acquired via the global [`HARDWARE`][crate::HARDWARE]
/// variable:
///
/// ```rust
/// let ai = HARDWARE.audio_interface.take();
/// ```
///
/// Once a reference has been acquired, registers can be accessed:
///
/// ```rust
/// let status = ai.status();
/// let length = ai.length_v1();
///
/// ai.set_dram_address(0x12345678)
///     .set_length_v1(123)
///     .enable_dma()
///     .set_dac_rate(DacRate::Ntsc);
/// ```
///
/// If needed, the reference can be given back to the global variable:
///
/// ```rust
/// ai.drop();
/// ```
#[non_exhaustive]
pub struct AudioInterface;

impl AudioInterface {
    /// Returns a reference to the audio interface registers.
    fn registers<'a>(&self) -> &'a AiRegisters {
        unsafe { &mut *(AI_REGS_BASE as *mut AiRegisters) }
    }

    /// Returns ownership of the audio interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.audio_interface.drop(self) }
    }

    /// Sets the DRAM address.
    pub fn set_dram_address(&self, dram_address: u32) -> &Self {
        self.registers()
            .dram_address
            .write(AiDmaAddress::ADDRESS.val(dram_address));
        self
    }

    /// Gets the transfer length (in v1 mode).
    pub fn length_v1(&self) -> u32 {
        self.registers().length.read(AiLength::TRANSFER_LENGTH_V1)
    }

    /// Sets the transfer length (in v1 mode).
    pub fn set_length_v1(&self, length: u32) -> &Self {
        self.registers()
            .length
            .write(AiLength::TRANSFER_LENGTH_V1.val(length));
        self
    }

    /// Gets the transfer length (in v2 mode).
    pub fn length_v2(&self) -> u32 {
        self.registers().length.read(AiLength::TRANSFER_LENGTH_V2)
    }

    /// Sets the transfer length (in v2 mode).
    pub fn set_length_v2(&self, length: u32) -> &Self {
        self.registers()
            .length
            .write(AiLength::TRANSFER_LENGTH_V2.val(length));
        self
    }

    /// Enables DMA.
    pub fn enable_dma(&self) -> &Self {
        self.registers().control.write(AiControl::DMA_ENABLE::SET);
        self
    }

    /// Disables DMA.
    pub fn disable_dma(&self) -> &Self {
        self.registers().control.write(AiControl::DMA_ENABLE::CLEAR);
        self
    }

    /// Gets the status.
    pub fn status(&self) -> u32 {
        todo!()
    }

    /// Sets the status.
    pub fn set_status(&self, _status: u32) -> &Self {
        todo!();
    }

    /// Sets the DAC rate.
    pub fn set_dac_rate(&self, dac_rate: DacRate) -> &Self {
        self.registers()
            .dacrate
            .write(AiDacRate::DAC_RATE.val(match dac_rate {
                DacRate::Ntsc => AiDacRate::DAC_RATE::Ntsc.into(),
                DacRate::Mpal => AiDacRate::DAC_RATE::Mpal.into(),
                DacRate::Pal => AiDacRate::DAC_RATE::Pal.into(),
            }));
        self
    }

    /// Sets the bit rate.
    pub fn set_bit_rate(&self, bit_rate: u32) -> &Self {
        self.registers()
            .bitrate
            .write(AiBitRate::BIT_RATE.val(bit_rate));
        self
    }
}

/// The DAC rate of the audio interface.
pub enum DacRate {
    /// Corresponds to a DAC rate of `0x2E6D354`.
    Ntsc,

    /// Corresponds to a DAC rate of `0x2F5B2D2`.
    Pal,

    /// Corresponds to a DAC rate of `0x2E6025C`.
    Mpal,
}

register_structs! {
    AiRegisters {
        (0x0000 => pub dram_address: WriteOnly<u32, AiDmaAddress::Register>),
        (0x0004 => pub length: ReadWrite<u32, AiLength::Register>),
        (0x0008 => pub control: WriteOnly<u32, AiControl::Register>),
        (0x000C => pub status: ReadWrite<u32, AiStatus::Register>),
        (0x0010 => pub dacrate: WriteOnly<u32, AiDacRate::Register>),
        (0x0014 => pub bitrate: WriteOnly<u32, AiBitRate::Register>),
        (0x0018 => @END),
    }
}

register_bitfields! {
    u32,

    AiDmaAddress [
        ADDRESS OFFSET(0) NUMBITS(24) [],
    ],

    AiLength [
        TRANSFER_LENGTH_V1 OFFSET(0)  NUMBITS(15) [],
        TRANSFER_LENGTH_V2 OFFSET(0)  NUMBITS(18) [],
    ],

    AiControl [
        DMA_ENABLE         OFFSET(0)  NUMBITS(1)  [],
    ],

    AiStatus [
        CLEAR_INTERRUPT    OFFSET(0)  NUMBITS(32) [],
        FULL               OFFSET(0)  NUMBITS(1)  [],
        DAC_COUNTER        OFFSET(1)  NUMBITS(14) [],
        BITCLOCK_STATE     OFFSET(16) NUMBITS(1)  [],
        ABUS_WORD_2        OFFSET(19) NUMBITS(1)  [],
        WORD_SELECT        OFFSET(21) NUMBITS(1)  [],
        DATA_AVAILABLE     OFFSET(22) NUMBITS(1)  [],
        DFIFO2_LOADED      OFFSET(23) NUMBITS(1)  [],
        DMA_ENABLE         OFFSET(25) NUMBITS(1)  [],
        DMA_REQUEST        OFFSET(26) NUMBITS(1)  [],
        DMA_BUSY           OFFSET(27) NUMBITS(1)  [],
        BUSY               OFFSET(30) NUMBITS(1)  [],
    ],

    AiDacRate [
        DAC_RATE           OFFSET(0)  NUMBITS(14) [
            Ntsc = 0x2E6D354,
            Pal  = 0x2F5B2D2,
            Mpal = 0x2E6025C,
        ],
    ],

    AiBitRate [
        BIT_RATE           OFFSET(0)  NUMBITS(4)  [],
    ],
}
