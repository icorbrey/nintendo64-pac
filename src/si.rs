//! # Serial Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

const SI_BASE_REG: u32 = 0x0480_0000;

/// Serial interface.
pub struct Si;

impl Si {
    pub fn ptr() -> *const SiRegisters {
        SI_BASE_REG as *const _
    }
}

unsafe impl Sync for Si {}

impl Deref for Si {
    type Target = SiRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Serial interface register block.
#[repr(C)]
pub struct SiRegisters {
    /// 0x00 - DRAM address
    pub si_dram_addr_reg: SiDramAddrReg,

    /// 0x04 - PIF address read 64 bits
    pub si_pif_addr_rd64b_reg: SiPifAddrRd64bReg,

    /// 0x08 - Reserved
    _reserved_0: u32,

    /// 0x0C - Reserved
    _reserved_1: u32,

    /// 0x10 - PIF address write 64 bits
    pub si_pif_addr_wr64b_reg: SiPifAddrWr64bReg,

    /// 0x14 - Reserved
    _reserved_2: u32,

    /// 0x18 - Status
    pub si_status_reg: SiStatusReg,
}

bitfield! {
    /// Serial interface DRAM address register.
    pub struct SiDramAddrReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub starting_rdram_address: u32 @ 0..24,
    }
}

bitfield! {
    /// Serial interface PIF address read 64 bits register.
    pub struct SiPifAddrRd64bReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub dma_write_64b: bool [write_only] @ 0,
    }
}

bitfield! {
    /// Serial interface PIF address write 64 bits register.
    pub struct SiPifAddrWr64bReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub dma_read_64b: bool [write_only] @ 0,
    }
}

bitfield! {
    /// Serial interface status register.
    pub struct SiStatusReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub dma_busy: bool [read_only] @ 0,
        pub io_read_busy: bool [read_only] @ 1,
        pub dma_error: bool [read_only] @ 3,
        pub interrupt: bool [read_only] @ 12,

        pub clear_intr: bool [write_only] @ 0,
    }
}
