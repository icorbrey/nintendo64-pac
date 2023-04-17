use tock_registers::{register_structs, registers::ReadWrite};

register_structs! {
    pub RdramRegisters {
        (0x0000 => pub config: ReadWrite<u32>),
        (0x0004 => pub device_id: ReadWrite<u32>),
        (0x0008 => pub delay: ReadWrite<u32>),
        (0x000C => pub mode: ReadWrite<u32>),
        (0x0010 => pub ref_interval: ReadWrite<u32>),
        (0x0014 => pub ref_row: ReadWrite<u32>),
        (0x0018 => pub ras_interval: ReadWrite<u32>),
        (0x001C => pub min_interval: ReadWrite<u32>),
        (0x0020 => pub address_select: ReadWrite<u32>),
        (0x0024 => pub device_manufacturer: ReadWrite<u32>),
        (0x0028 => @END),
    }
}
