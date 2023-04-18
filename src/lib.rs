#![no_std]

use core::mem::replace;

use ai::AudioInterface;
use dpc::Dpc;

pub mod ai;
pub mod dpc;
pub mod dps;
pub mod mi;
pub mod pi;
pub mod rdram;
pub mod ri;
pub mod si;
pub mod sp;
pub mod vi;

pub static mut HARDWARE: Hardware = Hardware {
    audio_interface: Peripheral::new(AudioInterface),
    dpc: Peripheral::new(Dpc),
};

pub struct Hardware {
    pub audio_interface: Peripheral<AudioInterface>,
    pub dpc: Peripheral<Dpc>,
}

pub struct Peripheral<T>(Option<T>);

impl<T> Peripheral<T> {
    pub const fn new(peripheral: T) -> Self {
        Self(Some(peripheral))
    }

    pub fn take(&mut self) -> Result<T, HardwareError> {
        let x = replace(&mut self.0, None);
        x.ok_or(HardwareError::TakePeripheralError)
    }

    pub fn drop(&mut self, peripheral: T) {
        self.0 = Some(peripheral);
    }
}

pub enum HardwareError {
    TakePeripheralError,
}
