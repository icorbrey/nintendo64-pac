//! # Nintendo 64 PAC
//!
//! Provides access to low-level Nintendo 64 memory in a type- and memory-safe
//! way.

// Don't include the standard library on the Nintendo 64.
#![cfg_attr(target_vendor = "nintendo64", no_std)]

use core::mem::replace;

use ai::AudioInterface;
use dpc::Dpc;
use dps::Dps;
use mi::MipsInterface;
use pc::ProgramCounter;
use pi::PeripheralInterface;
use rdram::Rdram;
use ri::RdramInterface;
use si::SerialInterface;
use sp::StackPointer;
use vi::VideoInterface;

pub mod ai;
pub mod dpc;
pub mod dps;
pub mod mi;
pub mod pc;
pub mod pi;
pub mod rdram;
pub mod ri;
pub mod si;
pub mod sp;
pub mod vi;

/// A global, static reference to the hardware peripherals of the Nintendo 64.
pub static mut HARDWARE: Hardware = Hardware {
    peripheral_interface: Peripheral::new(PeripheralInterface),
    serial_interface: Peripheral::new(SerialInterface),
    audio_interface: Peripheral::new(AudioInterface),
    program_counter: Peripheral::new(ProgramCounter),
    rdram_interface: Peripheral::new(RdramInterface),
    video_interface: Peripheral::new(VideoInterface),
    mips_interface: Peripheral::new(MipsInterface),
    stack_pointer: Peripheral::new(StackPointer),
    rdram: Peripheral::new(Rdram),
    dpc: Peripheral::new(Dpc),
    dps: Peripheral::new(Dps),
};

/// Contains references to each hardware peripheral on the system.
pub struct Hardware {
    /// Controlled reference to the peripheral interface.
    pub peripheral_interface: Peripheral<PeripheralInterface>,

    /// Controlled reference to the serial interface.
    pub serial_interface: Peripheral<SerialInterface>,

    /// Controlled reference to the audio interface.
    pub audio_interface: Peripheral<AudioInterface>,

    /// Controlled reference to the program counter.
    pub program_counter: Peripheral<ProgramCounter>,

    /// Controlled reference to the RDRAM interface.
    pub rdram_interface: Peripheral<RdramInterface>,

    /// Controlled reference to the video interface.
    pub video_interface: Peripheral<VideoInterface>,

    /// Controlled reference to the MIPS interface.
    pub mips_interface: Peripheral<MipsInterface>,

    /// Controlled reference to the stack pointer.
    pub stack_pointer: Peripheral<StackPointer>,

    /// Controlled reference to the RDRAM system.
    pub rdram: Peripheral<Rdram>,

    /// Controlled reference to the DPC system.
    pub dpc: Peripheral<Dpc>,

    /// Controlled reference to the DPS system.
    pub dps: Peripheral<Dps>,
}

/// An ownership wrapper for peripherals.
///
/// ```rust
/// # use nintendo64_pac::{HardwareError, Peripheral};
/// let mut foo = Peripheral::new(12345);
///
/// let bar = foo.take(); // Ok(12345)
/// # assert!(bar.is_ok());
/// let baz = foo.take(); // Err(HardwareError::TakePeripheralError)
/// # assert!(baz.is_err());
/// ```
pub struct Peripheral<T>(Option<T>);

impl<T> Peripheral<T> {
    /// Creates a new wrapper around the given peripheral.
    pub const fn new(peripheral: T) -> Self {
        Self(Some(peripheral))
    }

    /// Surrenders ownership of this peripheral to the caller.
    pub fn take(&mut self) -> Result<T, HardwareError> {
        replace(&mut self.0, None).ok_or(HardwareError::TakePeripheralError)
    }

    /// Takes ownership of the given peripheral back from the caller.
    pub fn drop(&mut self, peripheral: T) {
        self.0 = Some(peripheral);
    }
}

/// Errors related to the [`Hardware`][crate::Hardware] mechanism.
pub enum HardwareError {
    /// Occurs when an attempt is made to take a peripheral which has already
    /// been taken.
    TakePeripheralError,
}
