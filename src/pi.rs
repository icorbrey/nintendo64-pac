use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's peripheral interface registers.
#[cfg(target_vendor = "nintendo64")]
const PI_REGS_BASE: usize = 0x0460_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: PeripheralInterfaceRegisters = unsafe { std::mem::zeroed() };
}

#[non_exhaustive]
pub struct PeripheralInterface;

impl PeripheralInterface {
    /// Gets a reference to the peripheral interface registers.
    #[cfg(target_vendor = "nintendo64")]
    fn registers<'a>(&self) -> &'a PeripheralInterfaceRegisters {
        unsafe { &mut *(PI_REGS_BASE as *mut PeripheralInterfaceRegisters) }
    }

    /// Returns a reference to the peripheral interface registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the peripheral interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.peripheral_interface.drop(self) };
    }

    /// Gets whether DMA is busy.
    pub fn is_dma_busy(&self) -> bool {
        self.registers().status.is_set(Status::DMA_BUSY)
    }

    /// Gets whether IO is busy.
    pub fn is_io_busy(&self) -> bool {
        self.registers().status.is_set(Status::IO_BUSY)
    }

    /// Gets whether an error has occurred.
    pub fn has_error_occurred(&self) -> bool {
        self.registers().status.is_set(Status::ERROR)
    }

    /// Sets an address to read from.
    pub fn set_read_address(&self, address: u32) -> &Self {
        self.registers()
            .dram_address
            .write(ReadAddress::ADDRESS.val(address));
        self
    }

    /// Sets an address to write to.
    pub fn set_write_address(&self, address: u32) -> &Self {
        self.registers()
            .peripheral_address
            .write(WriteAddress::ADDRESS.val(address));
        self
    }

    /// Sets the length in bytes to transfer from `read_address` to
    /// `write_address`.
    pub fn set_read_length(&self, address: u32) -> &Self {
        self.registers()
            .read_length
            .write(Length::LENGTH.val(address));
        self
    }

    /// Clears an existing DP interrupt.
    pub fn clear_interrupt(&self) -> &Self {
        self.registers().status.write(Status::CLEAR_INTERRUPT::SET);
        self
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for PeripheralInterfaceRegisters {}

register_structs! {
    PeripheralInterfaceRegisters {
        (0x0000 => pub dram_address: ReadWrite<u32, ReadAddress::Register>),
        (0x0004 => pub peripheral_address: ReadWrite<u32, WriteAddress::Register>),
        (0x0008 => pub read_length: ReadWrite<u32, Length::Register>),
        (0x000C => pub write_length: ReadWrite<u32, Length::Register>),
        (0x0010 => pub status: ReadWrite<u32, Status::Register>),
        (0x0014 => pub domain1_latency: ReadWrite<u32, DeviceLatency::Register>),
        (0x0018 => pub domain1_pulse_width: ReadWrite<u32, DevicePulseWidth::Register>),
        (0x001C => pub domain1_page_size: ReadWrite<u32, DevicePageSize::Register>),
        (0x0020 => pub domain1_release: ReadWrite<u32, DeviceRelease::Register>),
        (0x0024 => pub domain2_latency: ReadWrite<u32, DeviceLatency::Register>),
        (0x0028 => pub domain2_pulse_width: ReadWrite<u32, DevicePulseWidth::Register>),
        (0x002C => pub domain2_page_size: ReadWrite<u32, DevicePageSize::Register>),
        (0x0030 => pub domain2_release: ReadWrite<u32, DeviceRelease::Register>),
        (0x0034 => @END),
    }
}

register_bitfields! {
    u32,

    ReadAddress [
        ADDRESS          OFFSET(0) NUMBITS(24) [],
    ],

    WriteAddress [
        ADDRESS          OFFSET(0) NUMBITS(32) [],
    ],

    Length [
        LENGTH           OFFSET(0) NUMBITS(24) [],
    ],

    Status [
        DMA_BUSY         OFFSET(0) NUMBITS(1)  [],
        IO_BUSY          OFFSET(1) NUMBITS(1)  [],
        ERROR            OFFSET(2) NUMBITS(1)  [],

        RESET_CONTROLLER OFFSET(0) NUMBITS(1)  [],
        CLEAR_INTERRUPT  OFFSET(1) NUMBITS(1)  [],
    ],

    DeviceLatency [
        LATENCY          OFFSET(0) NUMBITS(8)  [],
    ],

    DevicePulseWidth [
        PULSE_WIDTH      OFFSET(0) NUMBITS(8)  [],
    ],

    DevicePageSize [
        PAGE_SIZE        OFFSET(0) NUMBITS(4)  [],
    ],

    DeviceRelease [
        RELEASE          OFFSET(0) NUMBITS(2)  [],
    ],
}
