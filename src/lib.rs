#![no_std]

mod ai;
mod dpc;
mod dps;
mod mi;
mod pi;
mod rdram;
mod sp;
mod vi;

pub struct Hardware;

pub static mut HARDWARE: Hardware = Hardware;
