#![no_std]

use core::mem::replace;

use ai::AudioInterface;

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

pub struct Hardware {
    audio_interface: Option<AudioInterface>,
}

impl Hardware {
    pub fn take_audio_interface(&mut self) -> Result<AudioInterface, HardwareError> {
        let x = replace(&mut self.audio_interface, None);
        x.ok_or(HardwareError::TakePeripheralError)
    }

    pub fn drop_audio_interface(&mut self, audio_interface: AudioInterface) {
        self.audio_interface = Some(audio_interface);
    }
}

pub enum HardwareError {
    TakePeripheralError,
}

pub static mut HARDWARE: Hardware = Hardware {
    audio_interface: Some(AudioInterface),
};
