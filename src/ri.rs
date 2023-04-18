use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

const RI_REGS_BASE: usize = 0x0470_0000;

pub struct RdramInterface;

impl RdramInterface {
    pub fn registers<'a>(&self) -> &'a RdramInterfaceRegisters {
        unsafe { &mut *(RI_REGS_BASE as *mut RdramInterfaceRegisters) }
    }
}

register_structs! {
    pub RdramInterfaceRegisters {
        (0x0000 => pub mode: ReadWrite<u32, Mode::Register>),
        (0x0004 => pub config: ReadWrite<u32, Config::Register>),
        (0x0008 => pub current_load: WriteOnly<u32>),
        (0x000C => pub select: ReadWrite<u32, Select::Register>),
        (0x0010 => pub refresh: ReadWrite<u32, Refresh::Register>),
        (0x0014 => pub latency: ReadWrite<u32, Latency::Register>),
        (0x0018 => pub read_error: ReadOnly<u32, ReadError::Register>),
        (0x001C => pub write_error: WriteOnly<u32>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    pub Mode [
        OPERATING_MODE         OFFSET(0)  NUMBITS(2) [],
        STOP_TRANSMIT_ACTIVE   OFFSET(2)  NUMBITS(1) [],
        STOP_RECEIVE_ACTIVE    OFFSET(3)  NUMBITS(1) [],
    ],

    pub Config [
        CURRENT_CONTROL_INPUT  OFFSET(0)  NUMBITS(6) [],
        CURRENT_CONTROL_ENABLE OFFSET(6)  NUMBITS(1) [],
    ],

    pub Select [
        RECEIVE_SELECT         OFFSET(0)  NUMBITS(2) [],
        TRANSMIT_SELECT        OFFSET(2)  NUMBITS(2) [],
    ],

    pub Refresh [
        CLEAN_REFRESH_DELAY    OFFSET(0)  NUMBITS(8) [],
        DIRTY_REFRESH_DELAY    OFFSET(8)  NUMBITS(8) [],
        REFRESH_BANK           OFFSET(16) NUMBITS(1) [],
        REFRESH_ENABLE         OFFSET(17) NUMBITS(1) [],
        REFRESH_OPTIMIZE       OFFSET(18) NUMBITS(1) [],
    ],

    pub Latency [
        DMA_LATENCY            OFFSET(0)  NUMBITS(4) [],
    ],

    pub ReadError [
        NACK_ERROR             OFFSET(0)  NUMBITS(1) [],
        ACK_ERROR              OFFSET(1)  NUMBITS(1) [],
    ]
}
