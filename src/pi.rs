use tock_registers::{register_structs, registers::ReadWrite};

register_structs! {
    pub VideoInterfaceRegisters {
        (0x0000 => pub dram_address: ReadWrite<u32>),
        (0x0004 => pub cartridge_address: ReadWrite<u32>),
        (0x0008 => pub read_length: ReadWrite<u32>),
        (0x000C => pub write_length: ReadWrite<u32>),
        (0x0010 => pub status: ReadWrite<u32>),
        (0x0014 => pub domain1_latency: ReadWrite<u32>),
        (0x0018 => pub domain1_pulse_width: ReadWrite<u32>),
        (0x001C => pub domain1_page_size: ReadWrite<u32>),
        (0x0020 => pub domain1_release: ReadWrite<u32>),
        (0x0024 => pub domain2_latency: ReadWrite<u32>),
        (0x0028 => pub domain2_pulse_width: ReadWrite<u32>),
        (0x002C => pub domain2_page_size: ReadWrite<u32>),
        (0x0030 => pub domain2_release: ReadWrite<u32>),
        (0x0034 => @END),
    }
}
