use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

register_structs! {
    pub SerialInterfaceRegisters {
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
        ADDRESS      OFFSET(0)  NUMBITS(24) [],
    ],

    Status [
        DMA_BUSY     OFFSET(0)  NUMBITS(1)  [],
        IO_READ_BUSY OFFSET(1)  NUMBITS(1)  [],
        DMA_ERROR    OFFSET(3)  NUMBITS(1)  [],
        INTERRUPT    OFFSET(12) NUMBITS(1)  [],
    ],
}
