#![no_std]

mod rdram;

pub struct Hardware;

pub static mut HARDWARE: Hardware = Hardware;
