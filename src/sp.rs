//! # Stack Pointer Wrapper
//!
//! This module wraps the Nintendo 64's stack pointer registers and provides
//! type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::{register_access, HARDWARE};

register_access!(0x0404_0000, Registers);

/// A zero-size wrapper around the Nintendo 64's stack pointer system.
#[non_exhaustive]
pub struct StackPointer {
    /// Contains getters and setters for `SP_MEM_ADDR_REG`.
    pub memory_address: MemoryAddress,

    /// Contains getters and setters for `SP_DRAM_ADDR_REG`.
    pub dram_address: DramAddress,

    /// Contains getters and setters for `SP_WR_LEN_REG`.
    pub write_length: WriteLength,

    /// Contains getters and setters for `SP_RD_LEN_REG`.
    pub read_length: ReadLength,

    /// Contains getters and setters for `SP_SEMAPHORE_REG`.
    pub semaphore: Semaphore,

    /// Contains getters and setters for `SP_DMA_BUSY_REG`.
    pub dma_busy: DmaBusy,

    /// Contains getters and setters for `SP_DMA_FULL_REG`.
    pub dma_full: DmaFull,

    /// Contains getters and setters for `SP_STATUS_REG`.
    pub status: Status,
}

impl StackPointer {
    /// Returns ownership of the stack pointer registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.stack_pointer.drop(self) }
    }
}

/// A zero-size wrapper around `SP_MEM_ADDR_REG`.
#[non_exhaustive]
pub struct MemoryAddress;

impl MemoryAddress {
    pub fn get(&self) -> (u32, MemoryLocation) {
        let address = registers().mem_addr.read(SpMemAddrReg::ADDRESS);
        match registers().mem_addr.is_set(SpMemAddrReg::LOCATION) {
            false => (address, MemoryLocation::DataMemory),
            true => (address, MemoryLocation::InstructionMemory),
        }
    }

    pub fn set(&self, address: u32, location: MemoryLocation) {
        let location = match location {
            MemoryLocation::DataMemory => SpMemAddrReg::LOCATION::Dmem,
            MemoryLocation::InstructionMemory => SpMemAddrReg::LOCATION::Imem,
        };
        registers()
            .mem_addr
            .write(SpMemAddrReg::ADDRESS.val(address) + location)
    }
}

pub enum MemoryLocation {
    DataMemory,
    InstructionMemory,
}

/// A zero-size wrapper around `SP_DRAM_ADDR_REG`.
#[non_exhaustive]
pub struct DramAddress;

impl DramAddress {
    pub fn get(&self) -> u32 {
        registers().dram_addr.read(SpDramAddrReg::ADDRESS)
    }

    pub fn set(&self, address: u32) {
        registers()
            .dram_addr
            .write(SpDramAddrReg::ADDRESS.val(address))
    }
}

/// A zero-size wrapper around `SP_RD_LEN_REG`.
#[non_exhaustive]
pub struct ReadLength;

impl ReadLength {
    pub fn length(&self) -> u32 {
        registers().rd_len.read(SpRdLenReg::LENGTH)
    }

    pub fn count(&self) -> u32 {
        registers().rd_len.read(SpRdLenReg::COUNT)
    }

    pub fn skip(&self) -> u32 {
        registers().rd_len.read(SpRdLenReg::SKIP)
    }

    pub fn set_length(&self, length: u32) {
        registers().rd_len.write(SpRdLenReg::LENGTH.val(length))
    }

    pub fn set_count(&self, count: u32) {
        registers().rd_len.write(SpRdLenReg::COUNT.val(count))
    }

    pub fn set_skip(&self, skip: u32) {
        registers().rd_len.write(SpRdLenReg::SKIP.val(skip))
    }
}

/// A zero-size wrapper around `SP_WR_LEN_REG`.
#[non_exhaustive]
pub struct WriteLength;

impl WriteLength {
    pub fn length(&self) -> u32 {
        registers().wr_len.read(SpWrLenReg::LENGTH)
    }

    pub fn count(&self) -> u32 {
        registers().wr_len.read(SpWrLenReg::COUNT)
    }

    pub fn skip(&self) -> u32 {
        registers().wr_len.read(SpWrLenReg::SKIP)
    }

    pub fn set_length(&self, length: u32) {
        registers().wr_len.write(SpWrLenReg::LENGTH.val(length))
    }

    pub fn set_count(&self, count: u32) {
        registers().wr_len.write(SpWrLenReg::COUNT.val(count))
    }

    pub fn set_skip(&self, skip: u32) {
        registers().wr_len.write(SpWrLenReg::SKIP.val(skip))
    }
}

/// A zero-size wrapper around `SP_STATUS_REG`.
#[non_exhaustive]
pub struct Status;

impl Status {
    pub fn halt(&self) -> bool {
        registers().status.is_set(SpStatusReg::HALT)
    }

    pub fn broke(&self) -> bool {
        registers().status.is_set(SpStatusReg::BROKE)
    }

    pub fn dma_busy(&self) -> bool {
        registers().status.is_set(SpStatusReg::DMA_BUSY)
    }

    pub fn dma_full(&self) -> bool {
        registers().status.is_set(SpStatusReg::DMA_FULL)
    }

    pub fn io_full(&self) -> bool {
        registers().status.is_set(SpStatusReg::IO_FULL)
    }

    pub fn single_step(&self) -> bool {
        registers().status.is_set(SpStatusReg::SINGLE_STEP)
    }

    pub fn interrupt_on_break(&self) -> bool {
        registers().status.is_set(SpStatusReg::INTERRUPT_ON_BREAK)
    }

    pub fn signal_0_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_0_SET)
    }

    pub fn signal_1_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_1_SET)
    }

    pub fn signal_2_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_2_SET)
    }

    pub fn signal_3_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_3_SET)
    }

    pub fn signal_4_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_4_SET)
    }

    pub fn signal_5_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_5_SET)
    }

    pub fn signal_6_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_6_SET)
    }

    pub fn signal_7_set(&self) -> bool {
        registers().status.is_set(SpStatusReg::SIGNAL_7_SET)
    }

    pub fn clear_halt(&self) {
        registers().status.write(SpStatusReg::CLEAR_HALT::SET)
    }

    pub fn set_halt(&self) {
        registers().status.write(SpStatusReg::SET_HALT::SET)
    }

    pub fn clear_broke(&self) {
        registers().status.write(SpStatusReg::CLEAR_BROKE::SET)
    }

    pub fn clear_interrupt(&self) {
        registers().status.write(SpStatusReg::CLEAR_INTR::SET)
    }

    pub fn set_interrupt(&self) {
        registers().status.write(SpStatusReg::SET_INTR::SET)
    }

    pub fn clear_single_step(&self) {
        registers().status.write(SpStatusReg::CLEAR_SSTEP::SET)
    }

    pub fn set_single_step(&self) {
        registers().status.write(SpStatusReg::SET_SSTEP::SET)
    }

    pub fn clear_interrupt_on_break(&self) {
        registers()
            .status
            .write(SpStatusReg::CLEAR_INTR_ON_BREAK::SET)
    }

    pub fn set_interrupt_on_break(&self) {
        registers()
            .status
            .write(SpStatusReg::SET_INTR_ON_BREAK::SET)
    }

    pub fn clear_signal_0(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_0::SET)
    }

    pub fn set_signal_0(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_0::SET)
    }

    pub fn clear_signal_1(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_1::SET)
    }

    pub fn set_signal_1(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_1::SET)
    }

    pub fn clear_signal_2(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_2::SET)
    }

    pub fn set_signal_2(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_2::SET)
    }

    pub fn clear_signal_3(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_3::SET)
    }

    pub fn set_signal_3(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_3::SET)
    }

    pub fn clear_signal_4(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_4::SET)
    }

    pub fn set_signal_4(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_4::SET)
    }

    pub fn clear_signal_5(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_5::SET)
    }

    pub fn set_signal_5(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_5::SET)
    }

    pub fn clear_signal_6(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_6::SET)
    }

    pub fn set_signal_6(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_6::SET)
    }

    pub fn clear_signal_7(&self) {
        registers().status.write(SpStatusReg::CLEAR_SIGNAL_7::SET)
    }

    pub fn set_signal_7(&self) {
        registers().status.write(SpStatusReg::SET_SIGNAL_7::SET)
    }
}

/// A zero-size wrapper around `SP_DMA_FULL_REG`.
#[non_exhaustive]
pub struct DmaFull;

impl DmaFull {
    pub fn get(&self) -> bool {
        registers().dma_full.is_set(SpDmaFullReg::DMA_FULL)
    }
}

/// A zero-size wrapper around `SP_DMA_BUSY_REG`.
#[non_exhaustive]
pub struct DmaBusy;

impl DmaBusy {
    pub fn get(&self) -> bool {
        registers().dma_busy.is_set(SpDmaBusyReg::DMA_BUSY)
    }
}

/// A zero-size wrapper around `SP_SEMAPHORE_REG`.
#[non_exhaustive]
pub struct Semaphore;

impl Semaphore {
    pub fn get(&self) -> bool {
        registers().semaphore.is_set(SpSemaphoreReg::SEMAPHORE)
    }

    pub fn clear(&self) {
        registers().semaphore.write(SpSemaphoreReg::CLEAR::SET)
    }
}

register_structs! {
    Registers {
        (0x0000 => pub mem_addr: ReadWrite<u32, SpMemAddrReg::Register>),
        (0x0004 => pub dram_addr: ReadWrite<u32, SpDramAddrReg::Register>),
        (0x0008 => pub rd_len: ReadWrite<u32, SpRdLenReg::Register>),
        (0x000C => pub wr_len: ReadWrite<u32, SpWrLenReg::Register>),
        (0x0010 => pub status: ReadWrite<u32, SpStatusReg::Register>),
        (0x0014 => pub dma_full: ReadOnly<u32, SpDmaFullReg::Register>),
        (0x0018 => pub dma_busy: ReadOnly<u32, SpDmaBusyReg::Register>),
        (0x001C => pub semaphore: ReadWrite<u32, SpSemaphoreReg::Register>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    SpMemAddrReg [
        ADDRESS             OFFSET(0)  NUMBITS(12) [],
        LOCATION            OFFSET(12) NUMBITS(1)  [
            Dmem = 0,
            Imem = 1,
        ],
    ],

    SpDramAddrReg [
        ADDRESS             OFFSET(0)  NUMBITS(24) [],
    ],

    SpRdLenReg [
        LENGTH              OFFSET(0)  NUMBITS(12) [],
        COUNT               OFFSET(12) NUMBITS(8)  [],
        SKIP                OFFSET(20) NUMBITS(12) [],
    ],

    SpWrLenReg [
        LENGTH              OFFSET(0)  NUMBITS(12) [],
        COUNT               OFFSET(12) NUMBITS(8)  [],
        SKIP                OFFSET(20) NUMBITS(12) [],
    ],

    SpStatusReg [
        HALT                OFFSET(0)  NUMBITS(1)  [],
        BROKE               OFFSET(1)  NUMBITS(1)  [],
        DMA_BUSY            OFFSET(2)  NUMBITS(1)  [],
        DMA_FULL            OFFSET(3)  NUMBITS(1)  [],
        IO_FULL             OFFSET(4)  NUMBITS(1)  [],
        SINGLE_STEP         OFFSET(5)  NUMBITS(1)  [],
        INTERRUPT_ON_BREAK  OFFSET(6)  NUMBITS(1)  [],
        SIGNAL_0_SET        OFFSET(7)  NUMBITS(1)  [],
        SIGNAL_1_SET        OFFSET(8)  NUMBITS(1)  [],
        SIGNAL_2_SET        OFFSET(9)  NUMBITS(1)  [],
        SIGNAL_3_SET        OFFSET(10) NUMBITS(1)  [],
        SIGNAL_4_SET        OFFSET(11) NUMBITS(1)  [],
        SIGNAL_5_SET        OFFSET(12) NUMBITS(1)  [],
        SIGNAL_6_SET        OFFSET(13) NUMBITS(1)  [],
        SIGNAL_7_SET        OFFSET(14) NUMBITS(1)  [],

        CLEAR_HALT          OFFSET(0)  NUMBITS(1)  [],
        SET_HALT            OFFSET(1)  NUMBITS(1)  [],
        CLEAR_BROKE         OFFSET(2)  NUMBITS(1)  [],
        CLEAR_INTR          OFFSET(3)  NUMBITS(1)  [],
        SET_INTR            OFFSET(4)  NUMBITS(1)  [],
        CLEAR_SSTEP         OFFSET(5)  NUMBITS(1)  [],
        SET_SSTEP           OFFSET(6)  NUMBITS(1)  [],
        CLEAR_INTR_ON_BREAK OFFSET(7)  NUMBITS(1)  [],
        SET_INTR_ON_BREAK   OFFSET(8)  NUMBITS(1)  [],
        CLEAR_SIGNAL_0      OFFSET(9)  NUMBITS(1)  [],
        SET_SIGNAL_0        OFFSET(10) NUMBITS(1)  [],
        CLEAR_SIGNAL_1      OFFSET(11) NUMBITS(1)  [],
        SET_SIGNAL_1        OFFSET(12) NUMBITS(1)  [],
        CLEAR_SIGNAL_2      OFFSET(13) NUMBITS(1)  [],
        SET_SIGNAL_2        OFFSET(14) NUMBITS(1)  [],
        CLEAR_SIGNAL_3      OFFSET(15) NUMBITS(1)  [],
        SET_SIGNAL_3        OFFSET(16) NUMBITS(1)  [],
        CLEAR_SIGNAL_4      OFFSET(17) NUMBITS(1)  [],
        SET_SIGNAL_4        OFFSET(18) NUMBITS(1)  [],
        CLEAR_SIGNAL_5      OFFSET(19) NUMBITS(1)  [],
        SET_SIGNAL_5        OFFSET(10) NUMBITS(1)  [],
        CLEAR_SIGNAL_6      OFFSET(21) NUMBITS(1)  [],
        SET_SIGNAL_6        OFFSET(22) NUMBITS(1)  [],
        CLEAR_SIGNAL_7      OFFSET(23) NUMBITS(1)  [],
        SET_SIGNAL_7        OFFSET(24) NUMBITS(1)  [],
    ],

    SpDmaFullReg [
        DMA_FULL            OFFSET(0)  NUMBITS(1)  [],
    ],

    SpDmaBusyReg [
        DMA_BUSY            OFFSET(0)  NUMBITS(1)  [],
    ],

    SpSemaphoreReg [
        SEMAPHORE           OFFSET(0)  NUMBITS(1)  [],
        CLEAR               OFFSET(0)  NUMBITS(1)  [],
    ],
}
