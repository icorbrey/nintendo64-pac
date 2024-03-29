//! # Stack pointer (SP)

use proc_bitfield::bitfield;

use crate::{fields, registers};

/// # SP base address
pub const SP_BASE_REG: u32 = 0x0404_0000;

registers! {
    /// # Stack pointer (SP)
    SP_BASE_REG => Sp {
        /// DMEM/IMEM address
        pub sp_mem_addr_reg: SpMemAddrReg,

        /// DRAM address
        pub sp_dram_addr_reg: SpDramAddrReg,

        /// Read length
        pub sp_rd_len_reg: SpRdLenReg,

        /// Write length
        pub sp_wr_len_reg: SpWrLenReg,

        /// Status
        pub sp_status_reg: SpStatusReg,

        /// DMA full
        pub sp_dma_full_reg: SpDmaFullReg,

        /// DMA busy
        pub sp_dma_busy_reg: SpDmaBusyReg,

        /// Semaphore
        pub sp_semaphore_reg: SpSemaphoreReg,
    }
}

bitfield! {
    /// # SP DMEM/IMEM address register
    pub struct SpMemAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub mem_address: u16 [MemoryAddress] @ 0..12,
        pub dmem_imem: bool @ 12,
    }
}

bitfield! {
    /// # SP RDRAM address register
    pub struct SpDramAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub rdram_address: u32 [RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # SP read length register
    pub struct SpRdLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub length: u16 [Length] @ 0..12,
        pub count: u8 [Count] @ 12..20,
        pub skip: u16 [Skip] @ 20..32,
    }
}

bitfield! {
    /// # SP write length register
    pub struct SpWrLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub length: u16 [Length] @ 0..12,
        pub count: u8 [Count] @ 12..20,
        pub skip: u16 [Skip] @ 20..32,
    }
}

bitfield! {
    /// # SP status register
    pub struct SpStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,
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
    /// # SP DMA full register
    pub struct SpDmaFullReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_full: bool [read_only] @ 0,
    }
}

bitfield! {
    /// # SP DMA busy register
    pub struct SpDmaBusyReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_busy: bool [read_only] @ 0,
    }
}

bitfield! {
    /// # SP semaphore register
    pub struct SpSemaphoreReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub semaphore_flag: bool [read_only] @ 0,
        pub clear_semaphore_flag: bool [write_only] @ 0,
    }
}

fields! [
    /// # Memory address
    ux::u12 => MemoryAddress,

    /// # RDRAM address
    ux::u24 => RdramAddress,

    /// # Length
    ux::u12 => Length,

    /// # Count
    u8 => Count,

    /// # Skip
    ux::u12 => Skip,
];
