use tock_registers::{
    register_structs,
    registers::{ReadWrite, WriteOnly},
};

register_structs! {
    pub AudioInterfaceRegisters {
        (0x0000 => pub dram_address: WriteOnly<u32>),
        (0x0004 => pub length: ReadWrite<u32>),
        (0x0008 => pub control: WriteOnly<u32>),
        (0x000C => pub status: ReadWrite<u32>),
        (0x0010 => pub dacrate: WriteOnly<u32>),
        (0x0014 => pub bitrate: WriteOnly<u32>),
        (0x0018 => @END),
    }
}
