use tock_registers::{
    register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    pub DpcRegisters {
        (0x0000 => pub dma_start: ReadWrite<u32>),
        (0x0004 => pub dma_end: ReadWrite<u32>),
        (0x0008 => pub dma_current: ReadWrite<u32>),
        (0x000C => pub status: ReadWrite<u32>),
        (0x0010 => pub clock: ReadOnly<u32>),
        (0x0014 => pub buffer_busy: ReadOnly<u32>),
        (0x0018 => pub pipe_busy: ReadOnly<u32>),
        (0x001C => pub tmem: ReadOnly<u32>),
        (0x0020 => @END),
    }
}
