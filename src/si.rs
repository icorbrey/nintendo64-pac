use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's serial interface registers.
#[cfg(target_vendor = "nintendo64")]
const SI_REGS_BASE: usize = 0x0460_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: SerialInterfaceRegisters = unsafe { std::mem::zeroed() };
}

#[non_exhaustive]
pub struct SerialInterface;

impl SerialInterface {
    /// Gets a reference to the serial interface registers.
    #[cfg(target_vendor = "nintendo64")]
    pub fn registers<'a>(&self) -> &'a SerialInterfaceRegisters {
        unsafe { &mut *(SI_REGS_BASE as *mut SerialInterfaceRegisters) }
    }

    /// Returns a reference to the serial interface registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the serial interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.serial_interface.drop(self) }
    }

    /// Clears an existing interrupt.
    pub fn clear_interrupt(&self) -> &Self {
        self.registers().status.write(Status::CLEAR_INTERRUPT::SET);
        self
    }

    /// Returns whether DMA is currently busy.
    pub fn is_dma_busy(&self) -> bool {
        self.registers().status.is_set(Status::DMA_BUSY)
    }

    /// Returns whether IO is currently busy.
    pub fn is_io_busy(&self) -> bool {
        self.registers().status.is_set(Status::IO_BUSY)
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for SerialInterfaceRegisters {}

register_structs! {
    SerialInterfaceRegisters {
        (0x0000 => pub dram_address: ReadWrite<u32, DmaAddress::Register>),
        (0x0004 => pub read_64_bits: WriteOnly<u32>),
        (0x0008 => _reserved0),
        (0x0010 => pub write_64_bits: WriteOnly<u32>),
        (0x0014 => _reserved1),
        (0x0018 => pub status: ReadWrite<u32, Status::Register>),
        (0x001C => @END),
    }
}

register_bitfields! {
    u32,

    DmaAddress [
        ADDRESS         OFFSET(0)  NUMBITS(24) [],
    ],

    Status [
        DMA_BUSY        OFFSET(0)  NUMBITS(1)  [],
        IO_BUSY         OFFSET(1)  NUMBITS(1)  [],
        DMA_ERROR       OFFSET(3)  NUMBITS(1)  [],
        INTERRUPT       OFFSET(12) NUMBITS(1)  [],

        CLEAR_INTERRUPT OFFSET(0)  NUMBITS(1) [],
    ],
}
