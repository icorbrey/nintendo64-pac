use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

const SI_REGS_BASE: usize = 0x0460_0000;

pub struct SerialInterface;

impl SerialInterface {
    pub fn registers<'a>(&self) -> &'a SerialInterfaceRegisters {
        unsafe { &mut *(SI_REGS_BASE as *mut SerialInterfaceRegisters) }
    }
}

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

    pub DmaAddress [
        ADDRESS      OFFSET(0)  NUMBITS(24) [],
    ],

    pub Status [
        DMA_BUSY     OFFSET(0)  NUMBITS(1)  [],
        IO_READ_BUSY OFFSET(1)  NUMBITS(1)  [],
        DMA_ERROR    OFFSET(3)  NUMBITS(1)  [],
        INTERRUPT    OFFSET(12) NUMBITS(1)  [],
    ],
}
