use tock_registers::{register_structs, registers::ReadWrite};

register_structs! {
    pub VideoInterfaceRegisters {
        (0x0000 => pub status: ReadWrite<u32>),
        (0x0004 => pub origin: ReadWrite<u32>),
        (0x0008 => pub width: ReadWrite<u32>),
        (0x000C => pub vertical_interrupt: ReadWrite<u32>),
        (0x0010 => pub vertical_current: ReadWrite<u32>),
        (0x0014 => pub timing: ReadWrite<u32>),
        (0x0018 => pub vertical_sync: ReadWrite<u32>),
        (0x001C => pub horizontal_sync: ReadWrite<u32>),
        (0x0020 => pub leap: ReadWrite<u32>),
        (0x0024 => pub horizontal_video: ReadWrite<u32>),
        (0x0028 => pub vertical_video: ReadWrite<u32>),
        (0x002C => pub vertical_burst: ReadWrite<u32>),
        (0x0030 => pub x_scale: ReadWrite<u32>),
        (0x0034 => pub y_scale: ReadWrite<u32>),
        (0x0038 => @END),
    }
}
