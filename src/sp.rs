//! # Stack Pointer

use core::ops::Deref;

use proc_bitfield::bitfield;

const SP_BASE_REG: u32 = 0x0404_0000;

/// Stack pointer.
pub struct Sp;

impl Sp {
    pub fn ptr() -> *const SpRegisters {
        SP_BASE_REG as *const _
    }
}

impl Deref for Sp {
    type Target = SpRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Stack pointer register block.
#[repr(C)]
pub struct SpRegisters {
    /// `0x00` - DMEM/IMEM address
    pub sp_mem_addr_reg: SpMemAddrReg,

    /// `0x04` - DRAM address
    pub sp_dram_addr_reg: SpDramAddrReg,

    /// `0x08` - Read length
    pub sp_rd_len_reg: SpRdLenReg,

    /// `0x0C` - Write length
    pub sp_wr_len_reg: SpWrLenReg,

    /// `0x10` - Status
    pub sp_status_reg: SpStatusReg,

    /// `0x14` - DMA full
    pub sp_dma_full_reg: SpDmaFullReg,

    /// `0x18` - DMA busy
    pub sp_dma_busy_reg: SpDmaBusyReg,

    /// `0x1C` - Semaphore
    pub sp_semaphore_reg: SpSemaphoreReg,
}

bitfield! {
    /// Stack pointer DMEM/IMEM address register.
    pub struct SpMemAddrReg(pub u32): Debug {
        pub mem_address: u16 @ 0..12,
        pub dmem_imem: bool @ 12,
    }
}

bitfield! {
    /// Stack pointer RDRAM address register.
    pub struct SpDramAddrReg(pub u32): Debug {
        pub rdram_address: u32 @ 0..24,
    }
}

bitfield! {
    /// Stack pointer read length register.
    pub struct SpRdLenReg(pub u32): Debug {
        pub length: u16 @ 0..12,
        pub count: u8 @ 12..20,
        pub skip: u16 @ 20..32,
    }
}

bitfield! {
    /// Stack pointer write length register.
    pub struct SpWrLenReg(pub u32): Debug {
        pub length: u16 @ 0..12,
        pub count: u8 @ 12..20,
        pub skip: u16 @ 20..32,
    }
}

bitfield! {
    /// Stack pointer status register.
    pub struct SpStatusReg(pub u32): Debug {
        pub halt: bool [read_only] @ 0,
        pub broke: bool [read_only] @ 1,
        pub dma_busy: bool [read_only] @ 2,
        pub dma_full: bool [read_only] @ 3,
        pub io_full: bool [read_only] @ 4,
        pub single_step: bool [read_only] @ 5,
        pub intr_on_break: bool [read_only] @ 6,
        pub signal_0_set: bool [read_only] @ 7,
        pub signal_1_set: bool [read_only] @ 8,
        pub signal_2_set: bool [read_only] @ 9,
        pub signal_3_set: bool [read_only] @ 10,
        pub signal_4_set: bool [read_only] @ 11,
        pub signal_5_set: bool [read_only] @ 12,
        pub signal_6_set: bool [read_only] @ 13,
        pub signal_7_set: bool [read_only] @ 14,

        pub clear_halt: bool [write_only] @ 0,
        pub set_halt: bool [write_only] @ 1,
        pub clear_broke: bool [write_only] @ 2,
        pub clear_intr: bool [write_only] @ 3,
        pub set_intr: bool [write_only] @ 4,
        pub clear_sstep: bool [write_only] @ 5,
        pub set_sstep: bool [write_only] @ 6,
        pub clear_intr_on_break: bool [write_only] @ 7,
        pub set_intr_on_break: bool [write_only] @ 8,
        pub clear_signal_0: bool [write_only] @ 9,
        pub set_signal_0: bool [write_only] @ 10,
        pub clear_signal_1: bool [write_only] @ 11,
        pub set_signal_1: bool [write_only] @ 12,
        pub clear_signal_2: bool [write_only] @ 13,
        pub set_signal_2: bool [write_only] @ 14,
        pub clear_signal_3: bool [write_only] @ 15,
        pub set_signal_3: bool [write_only] @ 16,
        pub clear_signal_4: bool [write_only] @ 17,
        pub set_signal_4: bool [write_only] @ 18,
        pub clear_signal_5: bool [write_only] @ 19,
        pub set_signal_5: bool [write_only] @ 20,
        pub clear_signal_6: bool [write_only] @ 21,
        pub set_signal_6: bool [write_only] @ 22,
        pub clear_signal_7: bool [write_only] @ 23,
        pub set_signal_7: bool [write_only] @ 24,
    }
}

bitfield! {
    /// Stack pointer DMA full register.
    pub struct SpDmaFullReg(pub u32): Debug {
        pub dma_full: bool [read_only] @ 0,
    }
}

bitfield! {
    /// Stack pointer DMA busy register.
    pub struct SpDmaBusyReg(pub u32): Debug {
        pub dma_busy: bool [read_only] @ 0,
    }
}

bitfield! {
    /// Stack pointer semaphore register.
    pub struct SpSemaphoreReg(pub u32): Debug {
        /// Note: Sets on read.
        pub semaphore_flag: bool [read_only] @ 0,
        pub clear_semaphore_flag: bool [write_only] @ 0,
    }
}
