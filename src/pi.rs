use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

use crate::HARDWARE;

const PI_REGS_BASE: usize = 0x0460_0000;

pub struct PeripheralInterface;

impl PeripheralInterface {
    fn registers<'a>(&self) -> &'a PeripheralInterfaceRegisters {
        unsafe { &mut *(PI_REGS_BASE as *mut PeripheralInterfaceRegisters) }
    }

    pub fn drop(self) {
        unsafe { HARDWARE.peripheral_interface.drop(self) };
    }
}

register_structs! {
    pub PeripheralInterfaceRegisters {
        (0x0000 => pub dram_address: ReadWrite<u32, DmaAddress::Register>),
        (0x0004 => pub cartridge_address: ReadWrite<u32, CartridgeAddress::Register>),
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

    pub DmaAddress [
        ADDRESS          OFFSET(0) NUMBITS(24) [],
    ],

    pub CartridgeAddress [
        ADDRESS          OFFSET(0) NUMBITS(32) [],
    ],

    pub Length [
        DATA_LENGTH      OFFSET(0) NUMBITS(24) [],
    ],

    pub Status [
        DMA_BUSY         OFFSET(0) NUMBITS(1)  [],
        IO_BUSY          OFFSET(1) NUMBITS(1)  [],
        ERROR            OFFSET(2) NUMBITS(1)  [],

        RESET_CONTROLLER OFFSET(0) NUMBITS(1)  [],
        CLEAR_INTERRUPT  OFFSET(1) NUMBITS(1)  [],
    ],

    pub DeviceLatency [
        LATENCY          OFFSET(0) NUMBITS(8)  [],
    ],

    pub DevicePulseWidth [
        PULSE_WIDTH      OFFSET(0) NUMBITS(8)  [],
    ],

    pub DevicePageSize [
        PAGE_SIZE        OFFSET(0) NUMBITS(4)  [],
    ],

    pub DeviceRelease [
        RELEASE          OFFSET(0) NUMBITS(2)  [],
    ],
}
