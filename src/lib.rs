#![no_std]

mod ai;
mod dpc;
mod dps;
mod mi;
mod pi;
mod rdram;
mod ri;
mod si;
mod sp;
mod vi;

pub struct Hardware;

pub static mut HARDWARE: Hardware = Hardware;
