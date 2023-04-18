use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

register_structs! {
    pub VideoInterfaceRegisters {
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

    DmaAddress [
        ADDRESS          OFFSET(0) NUMBITS(24) [],
    ],

    CartridgeAddress [
        ADDRESS          OFFSET(0) NUMBITS(32) [],
    ],

    Length [
        DATA_LENGTH      OFFSET(0) NUMBITS(24) [],
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
