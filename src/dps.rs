use tock_registers::{register_structs, registers::ReadWrite};

register_structs! {
    pub DpcRegisters {
        (0x0000 => pub tmem_bist: ReadWrite<u32>),
        (0x0004 => pub test_mode: ReadWrite<u32>),
        (0x0008 => pub buffer_test_address: ReadWrite<u32>),
        (0x000C => pub buffer_test_data: ReadWrite<u32>),
        (0x0010 => @END),
    }
}
