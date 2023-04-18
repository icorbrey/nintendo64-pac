use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadWrite, WriteOnly},
};

const AI_REGS_BASE: usize = 0x0450_0000;

pub struct AudioInterface;

impl AudioInterface {
    pub fn registers<'a>(&self) -> &'a AudioInterfaceRegisters {
        unsafe { &mut *(AI_REGS_BASE as *mut AudioInterfaceRegisters) }
    }
}

register_structs! {
    pub AudioInterfaceRegisters {
        (0x0000 => pub dram_address: WriteOnly<u32, DmaAddress::Register>),
        (0x0004 => pub length: ReadWrite<u32, Length::Register>),
        (0x0008 => pub control: WriteOnly<u32, Control::Register>),
        (0x000C => pub status: ReadWrite<u32, Status::Register>),
        (0x0010 => pub dacrate: WriteOnly<u32, DacRate::Register>),
        (0x0014 => pub bitrate: WriteOnly<u32, BitRate::Register>),
        (0x0018 => @END),
    }
}

register_bitfields! {
    u32,

    pub DmaAddress [
        ADDRESS OFFSET(0) NUMBITS(24) [],
    ],

    pub Length [
        TRANSFER_LENGTH_V1 OFFSET(0)  NUMBITS(15) [],
        TRANSFER_LENGTH_V2 OFFSET(0)  NUMBITS(18) [],
    ],

    pub Control [
        DMA_ENABLE         OFFSET(0)  NUMBITS(1)  [],
    ],

    pub Status [
        CLEAR_INTERRUPT    OFFSET(0)  NUMBITS(32) [],
        FULL               OFFSET(0)  NUMBITS(1)  [],
        DAC_COUNTER        OFFSET(1)  NUMBITS(14) [],
        BITCLOCK_STATE     OFFSET(16) NUMBITS(1)  [],
        ABUS_WORD_2        OFFSET(19) NUMBITS(1)  [],
        WORD_SELECT        OFFSET(21) NUMBITS(1)  [],
        DATA_AVAILABLE     OFFSET(22) NUMBITS(1)  [],
        DFIFO2_LOADED      OFFSET(23) NUMBITS(1)  [],
        DMA_ENABLE         OFFSET(25) NUMBITS(1)  [],
        DMA_REQUEST        OFFSET(26) NUMBITS(1)  [],
        DMA_BUSY           OFFSET(27) NUMBITS(1)  [],
        BUSY               OFFSET(30) NUMBITS(1)  [],
    ],

    pub DacRate [
        DAC_RATE           OFFSET(0)  NUMBITS(14) [
            Ntsc = 0x2E6D354,
            Pal  = 0x2F5B2D2,
            Mpal = 0x2E6025C,
        ],
    ],

    pub BitRate [
        BIT_RATE           OFFSET(0)  NUMBITS(4)  [],
    ],
}
