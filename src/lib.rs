#![no_std]

mod dpc;
mod dps;
mod rdram;
mod sp;

pub struct Hardware;

pub static mut HARDWARE: Hardware = Hardware;
