use tock_registers::{
    register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    pub MipsInterfaceRegisters {
        (0x0000 => pub init_mode: ReadWrite<u32>),
        (0x0004 => pub version: ReadOnly<u32>),
        (0x0008 => pub interrupt: ReadOnly<u32>),
        (0x000C => pub interrupt_mask: ReadWrite<u32>),
        (0x0010 => @END),
    }
}
