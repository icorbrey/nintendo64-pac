//! # Serial interface (SI)

use proc_bitfield::bitfield;

use crate::{fields, registers};

/// # SI base address
pub const SI_BASE_REG: u32 = 0x0480_0000;

registers! {
    /// # Serial interface (SI)
    SI_BASE_REG => Si {
        /// DRAM address
        pub si_dram_addr_reg: SiDramAddrReg,

        /// PIF address read 64 bits
        pub si_pif_addr_rd64b_reg: SiPifAddrRd64bReg,

        /// Reserved
        _reserved_0: u32,

        /// Reserved
        _reserved_1: u32,

        /// PIF address write 64 bits
        pub si_pif_addr_wr64b_reg: SiPifAddrWr64bReg,

        /// Reserved
        _reserved_2: u32,

        /// Status
        pub si_status_reg: SiStatusReg,
    }
}

bitfield! {
    /// # SI DRAM address register
    pub struct SiDramAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_rdram_address: u32 [RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # SI PIF address read 64 bits register
    pub struct SiPifAddrRd64bReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_write_64b: bool [write_only] @ 0,
    }
}

bitfield! {
    /// # SI PIF address write 64 bits register
    pub struct SiPifAddrWr64bReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_read_64b: bool [write_only] @ 0,
    }
}

bitfield! {
    /// # SI status register
    pub struct SiStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_busy: bool [read_only] @ 0,
        pub io_read_busy: bool [read_only] @ 1,
        pub dma_error: bool [read_only] @ 3,
        pub interrupt: bool [read_only] @ 12,
        pub clear_intr: bool [write_only] @ 0,
    }
}

fields! [
    /// # RDRAM address
    ux::u24 => RdramAddress,
];
