#![no_std]

mod rdram;
mod sp;

pub struct Hardware;

pub static mut HARDWARE: Hardware = Hardware;
