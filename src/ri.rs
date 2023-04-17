use tock_registers::{
    register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

register_structs! {
    pub RdramRegisters {
        (0x0000 => pub mode: ReadWrite<u32>),
        (0x0004 => pub config: ReadWrite<u32>),
        (0x0008 => pub current_load: WriteOnly<u32>),
        (0x000C => pub select: ReadWrite<u32>),
        (0x0010 => pub refresh: ReadWrite<u32>),
        (0x0014 => pub latency: ReadWrite<u32>),
        (0x0018 => pub read_error: ReadOnly<u32>),
        (0x001C => pub write_error: WriteOnly<u32>),
        (0x0020 => @END),
    }
}
