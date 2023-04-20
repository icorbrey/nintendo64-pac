use tock_registers::{
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

const SP_REGS_BASE: usize = 0x0404_0000;

#[non_exhaustive]
pub struct StackPointer;

impl StackPointer {
    fn registers<'a>(&self) -> &'a StackPointerRegisters {
        unsafe { &mut *(SP_REGS_BASE as *mut StackPointerRegisters) }
    }
}

register_structs! {
    StackPointerRegisters {
        (0x0000 => pub memory_address: ReadWrite<u32, MemoryAddress::Register>),
        (0x0004 => pub dma_address: ReadWrite<u32, DmaAddress::Register>),
        (0x0008 => pub read_dma_length: ReadWrite<u32, DmaLength::Register>),
        (0x000C => pub write_dma_length: ReadWrite<u32, DmaLength::Register>),
        (0x0010 => pub status: ReadWrite<u32, Status::Register>),
        (0x0014 => pub dma_full: ReadOnly<u32, DmaFull::Register>),
        (0x0018 => pub dma_busy: ReadOnly<u32, DmaBusy::Register>),
        (0x001C => pub semaphore: ReadOnly<u32, Semaphore::Register>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    MemoryAddress [
        ADDRESS                  OFFSET(0)  NUMBITS(12) [],
        LOCATION                 OFFSET(12) NUMBITS(1)  [
            DataMemory = 0,
            InstructionMemory = 1,
        ],
    ],

    DmaAddress [
        ADDRESS                  OFFSET(0)  NUMBITS(24) [],
    ],

    DmaLength [
        LENGTH                   OFFSET(0)  NUMBITS(12) [],
        COUNT                    OFFSET(12) NUMBITS(8)  [],
        SKIP                     OFFSET(20) NUMBITS(12) [],
    ],

    Status [
        HALT                     OFFSET(0)  NUMBITS(1)  [],
        BROKE                    OFFSET(1)  NUMBITS(1)  [],
        DMA_BUSY                 OFFSET(2)  NUMBITS(1)  [],
        DMA_FULL                 OFFSET(3)  NUMBITS(1)  [],
        IO_FULL                  OFFSET(4)  NUMBITS(1)  [],
        SINGLE_STEP              OFFSET(5)  NUMBITS(1)  [],
        INTERRUPT_ON_BREAK       OFFSET(6)  NUMBITS(1)  [],
        SIGNAL_0_SET             OFFSET(7)  NUMBITS(1)  [],
        SIGNAL_1_SET             OFFSET(8)  NUMBITS(1)  [],
        SIGNAL_2_SET             OFFSET(9)  NUMBITS(1)  [],
        SIGNAL_3_SET             OFFSET(10) NUMBITS(1)  [],
        SIGNAL_4_SET             OFFSET(11) NUMBITS(1)  [],
        SIGNAL_5_SET             OFFSET(12) NUMBITS(1)  [],
        SIGNAL_6_SET             OFFSET(13) NUMBITS(1)  [],
        SIGNAL_7_SET             OFFSET(14) NUMBITS(1)  [],

        CLEAR_HALT               OFFSET(0)  NUMBITS(1)  [],
        SET_HALT                 OFFSET(1)  NUMBITS(1)  [],
        CLEAR_BROKE              OFFSET(2)  NUMBITS(1)  [],
        CLEAR_INTERRUPT          OFFSET(3)  NUMBITS(1)  [],
        SET_INTERRUPT            OFFSET(4)  NUMBITS(1)  [],
        CLEAR_SINGLE_STEP        OFFSET(5)  NUMBITS(1)  [],
        SET_SINGLE_STEP          OFFSET(6)  NUMBITS(1)  [],
        CLEAR_INTERRUPT_ON_BREAK OFFSET(7)  NUMBITS(1)  [],
        SET_INTERRUPT_ON_BREAK   OFFSET(8)  NUMBITS(1)  [],
        CLEAR_SIGNAL_0           OFFSET(9)  NUMBITS(1)  [],
        SET_SIGNAL_0             OFFSET(10) NUMBITS(1)  [],
        CLEAR_SIGNAL_1           OFFSET(11) NUMBITS(1)  [],
        SET_SIGNAL_1             OFFSET(12) NUMBITS(1)  [],
        CLEAR_SIGNAL_2           OFFSET(13) NUMBITS(1)  [],
        SET_SIGNAL_2             OFFSET(14) NUMBITS(1)  [],
        CLEAR_SIGNAL_3           OFFSET(15) NUMBITS(1)  [],
        SET_SIGNAL_3             OFFSET(16) NUMBITS(1)  [],
        CLEAR_SIGNAL_4           OFFSET(17) NUMBITS(1)  [],
        SET_SIGNAL_4             OFFSET(18) NUMBITS(1)  [],
        CLEAR_SIGNAL_5           OFFSET(19) NUMBITS(1)  [],
        SET_SIGNAL_5             OFFSET(10) NUMBITS(1)  [],
        CLEAR_SIGNAL_6           OFFSET(21) NUMBITS(1)  [],
        SET_SIGNAL_6             OFFSET(22) NUMBITS(1)  [],
        CLEAR_SIGNAL_7           OFFSET(23) NUMBITS(1)  [],
        SET_SIGNAL_7             OFFSET(24) NUMBITS(1)  [],
    ],

    DmaFull [
        DMA_FULL                 OFFSET(0)  NUMBITS(1)  [],
    ],

    DmaBusy [
        DMA_BUSY                 OFFSET(0)  NUMBITS(1)  [],
    ],

    Semaphore [
        SEMAPHORE                OFFSET(0)  NUMBITS(1)  [],
    ],
}
