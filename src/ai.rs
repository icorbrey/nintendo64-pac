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
/// let is_busy = ai.is_busy();
/// let is_full = ai.is_full();
///
/// ai.set_sample_buffer_address(0x12345678)
///     .set_sample_buffer_length(123)
///     .set_dac_rate(0x12345678)
///     .start_sample_playback();
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
    fn registers<'a>(&self) -> &'a AudioInterfaceRegisters {
        unsafe { &mut *(AI_REGS_BASE as *mut AudioInterfaceRegisters) }
    }

    /// Returns ownership of the audio interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.audio_interface.drop(self) }
    }

    /// Sets the address of the uncached memory buffer of samples to play.
    pub fn set_sample_buffer_address(&self, sample_buffer_address: u32) -> &Self {
        self.registers()
            .sample_buffer_address
            .write(SampleBufferAddress::ADDRESS.val(sample_buffer_address));
        self
    }

    /// Gets the length of the sample buffer.
    pub fn sample_buffer_length_v1(&self) -> u32 {
        self.registers()
            .sample_buffer_length
            .read(SampleBufferLength::SAMPLE_BUFFER_LENGTH_V1)
    }

    /// Sets the length of the sample buffer.
    pub fn set_sample_buffer_length_v1(&self, bytes: u32) -> &Self {
        self.registers()
            .sample_buffer_length
            .write(SampleBufferLength::SAMPLE_BUFFER_LENGTH_V1.val(bytes));
        self
    }

    /// Gets the length of the sample buffer.
    pub fn sample_buffer_length_v2(&self) -> u32 {
        self.registers()
            .sample_buffer_length
            .read(SampleBufferLength::SAMPLE_BUFFER_LENGTH_V2)
    }

    /// Sets the length of the sample buffer.
    pub fn set_sample_buffer_length_v2(&self, bytes: u32) -> &Self {
        self.registers()
            .sample_buffer_length
            .write(SampleBufferLength::SAMPLE_BUFFER_LENGTH_V2.val(bytes));
        self
    }

    /// Starts playback of an audio sample.
    pub fn start_sample_playback(&self) -> &Self {
        self.registers()
            .sample_playback
            .write(SamplePlaybackControl::START::SET);
        self
    }

    /// Gets whether the audio interface is currently full.
    pub fn is_full(&self) -> bool {
        self.registers().status.is_set(Status::FULL)
    }

    /// Gets whether the audio interface is currently busy.
    pub fn is_busy(&self) -> bool {
        self.registers().status.is_set(Status::BUSY)
    }

    /// Clears the audio interface interrupt.
    pub fn clear_interrupt(&self) -> &Self {
        self.registers().status.write(Status::CLEAR_INTERRUPT::SET);
        self
    }

    /// Sets the rate at which the sample buffer should be played.
    pub fn set_dac_rate(&self, dac_rate: u32) -> &Self {
        self.registers().dacrate.set(dac_rate);
        self
    }

    /// Sets the length of a single sample in bits.
    pub fn set_sample_length(&self, bits: u32) -> &Self {
        self.registers().sample_length.set(bits);
        self
    }
}

register_structs! {
    AudioInterfaceRegisters {
        (0x0000 => pub sample_buffer_address: WriteOnly<u32, SampleBufferAddress::Register>),
        (0x0004 => pub sample_buffer_length: ReadWrite<u32, SampleBufferLength::Register>),
        (0x0008 => pub sample_playback: WriteOnly<u32, SamplePlaybackControl::Register>),
        (0x000C => pub status: ReadWrite<u32, Status::Register>),
        (0x0010 => pub dacrate: WriteOnly<u32>),
        (0x0014 => pub sample_length: WriteOnly<u32>),
        (0x0018 => @END),
    }
}

register_bitfields! {
    u32,

    SampleBufferAddress [
        ADDRESS                 OFFSET(0)  NUMBITS(24) [],
    ],

    SampleBufferLength [
        SAMPLE_BUFFER_LENGTH_V1 OFFSET(0)  NUMBITS(15) [],
        SAMPLE_BUFFER_LENGTH_V2 OFFSET(0)  NUMBITS(18) [],
    ],

    SamplePlaybackControl [
        START                   OFFSET(0)  NUMBITS(1)  [],
    ],

    Status [
        BUSY                    OFFSET(30) NUMBITS(1)  [],
        FULL                    OFFSET(31) NUMBITS(1)  [],

        CLEAR_INTERRUPT         OFFSET(0)  NUMBITS(1)  [],
    ],
}
