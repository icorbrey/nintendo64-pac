use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    pub DpcRegisters {
        (0x0000 => pub dma_start: ReadWrite<u32, DmaAddress::Register>),
        (0x0004 => pub dma_end: ReadWrite<u32, DmaAddress::Register>),
        (0x0008 => pub dma_current: ReadWrite<u32, DmaAddress::Register>),
        (0x000C => pub status: ReadWrite<u32, Status::Register>),
        (0x0010 => pub clock: ReadOnly<u32, ClockCounter::Register>),
        (0x0014 => pub buffer_busy: ReadOnly<u32, ClockCounter::Register>),
        (0x0018 => pub pipe_busy: ReadOnly<u32, ClockCounter::Register>),
        (0x001C => pub tmem: ReadOnly<u32, ClockCounter::Register>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    DmaAddress [
        ADDRESS       OFFSET(0)  NUMBITS(24) [],
    ],

    Status [
        XBUS_DMEM_DMA OFFSET(0)  NUMBITS(1)  [],
        FREEZE        OFFSET(1)  NUMBITS(1)  [],
        FLUSH         OFFSET(2)  NUMBITS(1)  [],
        START_GLCK    OFFSET(3)  NUMBITS(1)  [],
        TMEM_BUSY     OFFSET(4)  NUMBITS(1)  [],
        PIPE_BUSY     OFFSET(5)  NUMBITS(1)  [],
        CMD_BUSY      OFFSET(6)  NUMBITS(1)  [],
        CBUF_READY    OFFSET(7)  NUMBITS(1)  [],
        DMA_BUSY      OFFSET(8)  NUMBITS(1)  [],
        END_VALID     OFFSET(9)  NUMBITS(1)  [],
        START_VALID   OFFSET(10) NUMBITS(1)  [],
    ],

    ClockCounter [
        CLOCK_COUNTER OFFSET(0)  NUMBITS(24) [],
    ]
}
