#![no_std]

mod dpc;
mod dps;
mod mi;
mod rdram;
mod sp;
mod vi;

pub struct Hardware;

pub static mut HARDWARE: Hardware = Hardware;
