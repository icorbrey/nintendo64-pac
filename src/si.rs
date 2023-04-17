use tock_registers::{
    register_structs,
    registers::{ReadWrite, WriteOnly},
};

register_structs! {
    pub SerialInterfaceRegisters {
        (0x0000 => pub dram_address: ReadWrite<u32>),
        (0x0004 => pub read_64_bits: WriteOnly<u32>),
        (0x0008 => pub write_64_bits: WriteOnly<u32>),
        (0x000C => pub status: ReadWrite<u32>),
        (0x0010 => @END),
    }
}
