use tock_registers::{
    register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    pub StackPointerRegisters {
        (0x0000 => pub memory_address: ReadWrite<u32>),
        (0x0004 => pub dma_address: ReadWrite<u32>),
        (0x0008 => pub read_dma_length: ReadWrite<u32>),
        (0x000C => pub write_dma_length: ReadWrite<u32>),
        (0x0010 => pub status: ReadWrite<u32>),
        (0x0014 => pub dma_full: ReadOnly<u32>),
        (0x0018 => pub dma_busy: ReadOnly<u32>),
        (0x001C => pub semaphore: ReadOnly<u32>),
        (0x0020 => @END),
    }
}

register_structs! {
    pub ProgramCounterRegisters {
        (0x0000 => pub program_counter: ReadWrite<u32>),
        (0x0004 => pub imem_bist: ReadWrite<u32>),
        (0x0008 => @END),
    }
}
