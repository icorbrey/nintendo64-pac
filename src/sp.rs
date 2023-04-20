use tock_registers::{
    interfaces::Writeable,
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's stack pointer registers.
#[cfg(target_vendor = "nintendo64")]
const SP_REGS_BASE: usize = 0x0404_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: StackPointerRegisters = unsafe { std::mem::zeroed() };
}

#[non_exhaustive]
pub struct StackPointer;

impl StackPointer {
    /// Gets a reference to the stack pointer registers.
    #[cfg(target_vendor = "nintendo64")]
    fn registers<'a>(&self) -> &'a StackPointerRegisters {
        unsafe { &mut *(SP_REGS_BASE as *mut StackPointerRegisters) }
    }

    /// Gets a reference to the stack pointer registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the stack pointer registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.stack_pointer.drop(self) }
    }

    /// Clears an existing interrupt.
    pub fn clear_interrupt(&self) -> &Self {
        self.registers().status.write(Status::CLEAR_INTERRUPT::SET);
        self
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for StackPointerRegisters {}

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
