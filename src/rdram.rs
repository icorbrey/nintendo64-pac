//! # RDRAM Wrapper
//!
//! This module wraps the Nintendo 64's RDRAM registers and provides type- and
//! memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::ReadWrite,
};

use crate::{register_access, HARDWARE};

register_access!(0x03F0_0000, Registers);

#[non_exhaustive]
pub struct Rdram {
    pub device_manufacturer: DeviceManufacturer,
    pub address_select: AddressSelect,
    pub min_interval: MinInterval,
    pub ras_interval: RasInterval,
    pub ref_interval: RefInterval,
    pub device_id: DeviceId,
    pub ref_row: RefRow,
    pub config: Config,
    pub delay: Delay,
    pub mode: Mode,
}

impl Rdram {
    /// Returns ownership of the RDRAM registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.rdram.drop(self) }
    }
}

/// A zero-size wrapper around `RDRAM_CONFIG_REG`.
#[non_exhaustive]
pub struct Config;

impl Config {
    pub fn get(&self) -> u32 {
        registers().config.get()
    }

    pub fn set(&self, config: u32) {
        registers().config.set(config)
    }
}

/// A zero-size wrapper around `RDRAM_DEVICE_ID_REG`.
#[non_exhaustive]
pub struct DeviceId;

impl DeviceId {
    pub fn get(&self) -> u32 {
        registers().device_id.get()
    }

    pub fn set(&self, device_id: u32) {
        registers().config.set(device_id)
    }
}

/// A zero-size wrapper around `RDRAM_DELAY_REG`.
#[non_exhaustive]
pub struct Delay;

impl Delay {
    pub fn get(&self) -> u32 {
        registers().delay.get()
    }

    pub fn set(&self, delay: u32) {
        registers().config.set(delay)
    }
}

/// A zero-size wrapper around `RDRAM_MODE_REG`.
#[non_exhaustive]
pub struct Mode;

impl Mode {
    pub fn get(&self) -> u32 {
        registers().mode.get()
    }

    pub fn set(&self, mode: u32) {
        registers().config.set(mode)
    }
}

/// A zero-size wrapper around `RDRAM_REF_INTERVAL_REG`.
#[non_exhaustive]
pub struct RefInterval;

impl RefInterval {
    pub fn get(&self) -> u32 {
        registers().ref_interval.get()
    }

    pub fn set(&self, ref_interval: u32) {
        registers().config.set(ref_interval)
    }
}

/// A zero-size wrapper around `RDRAM_REF_ROW_REG`.
#[non_exhaustive]
pub struct RefRow;

impl RefRow {
    pub fn get(&self) -> u32 {
        registers().ref_row.get()
    }

    pub fn set(&self, ref_row: u32) {
        registers().config.set(ref_row)
    }
}

/// A zero-size wrapper around `RDRAM_RAS_INTERVAL_REG`.
#[non_exhaustive]
pub struct RasInterval;

impl RasInterval {
    pub fn get(&self) -> u32 {
        registers().ras_interval.get()
    }

    pub fn set(&self, ras_interval: u32) {
        registers().config.set(ras_interval)
    }
}

/// A zero-size wrapper around `RDRAM_MIN_INTERVAL_REG`.
#[non_exhaustive]
pub struct MinInterval;

impl MinInterval {
    pub fn get(&self) -> u32 {
        registers().min_interval.get()
    }

    pub fn set(&self, min_interval: u32) {
        registers().config.set(min_interval)
    }
}

/// A zero-size wrapper around `RDRAM_ADDR_SELECT_REG`.
#[non_exhaustive]
pub struct AddressSelect;

impl AddressSelect {
    pub fn get(&self) -> u32 {
        registers().addr_select.get()
    }

    pub fn set(&self, address_select: u32) {
        registers().config.set(address_select)
    }
}

/// A zero-size wrapper around `RDRAM_DEVICE_MANUF_REG`.
#[non_exhaustive]
pub struct DeviceManufacturer;

impl DeviceManufacturer {
    pub fn get(&self) -> u32 {
        registers().device_manuf.get()
    }

    pub fn set(&self, device_manufacturer: u32) {
        registers().config.set(device_manufacturer)
    }
}

register_structs! {
    Registers {
        (0x0000 => pub config: ReadWrite<u32>),
        (0x0004 => pub device_id: ReadWrite<u32>),
        (0x0008 => pub delay: ReadWrite<u32>),
        (0x000C => pub mode: ReadWrite<u32>),
        (0x0010 => pub ref_interval: ReadWrite<u32>),
        (0x0014 => pub ref_row: ReadWrite<u32>),
        (0x0018 => pub ras_interval: ReadWrite<u32>),
        (0x001C => pub min_interval: ReadWrite<u32>),
        (0x0020 => pub addr_select: ReadWrite<u32>),
        (0x0024 => pub device_manuf: ReadWrite<u32>),
        (0x0028 => @END),
    }
}
