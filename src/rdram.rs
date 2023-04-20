use tock_registers::{register_structs, registers::ReadWrite};

use crate::HARDWARE;

/// The static address of the Nintendo 64's RDRAM registers.
#[cfg(target_vendor = "nintendo64")]
const RDRAM_REGS_BASE: usize = 0x03F0_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: RdramRegisters = unsafe { std::mem::zeroed() };
}

#[non_exhaustive]
pub struct Rdram;

impl Rdram {
    /// Gets a reference to the RDRAM registers.
    #[cfg(target_vendor = "nintendo64")]
    pub fn registers<'a>(&self) -> &'a RdramRegisters {
        unsafe { &mut *(RDRAM_REGS_BASE as *mut RdramRegisters) }
    }

    /// Returns a reference to the RDRAM registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the RDRAM registers to [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.rdram.drop(self) }
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for RdramRegisters {}

register_structs! {
    RdramRegisters {
        (0x0000 => pub config: ReadWrite<u32>),
        (0x0004 => pub device_id: ReadWrite<u32>),
        (0x0008 => pub delay: ReadWrite<u32>),
        (0x000C => pub mode: ReadWrite<u32>),
        (0x0010 => pub ref_interval: ReadWrite<u32>),
        (0x0014 => pub ref_row: ReadWrite<u32>),
        (0x0018 => pub ras_interval: ReadWrite<u32>),
        (0x001C => pub min_interval: ReadWrite<u32>),
        (0x0020 => pub address_select: ReadWrite<u32>),
        (0x0024 => pub device_manufacturer: ReadWrite<u32>),
        (0x0028 => @END),
    }
}
