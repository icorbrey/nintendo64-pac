//! # Peripheral Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

const PI_BASE_REG: u32 = 0x0460_0000;

/// Peripheral interface.
pub struct Pi;

impl Pi {
    pub fn ptr() -> *const PiRegisters {
        PI_BASE_REG as *const _
    }
}

unsafe impl Sync for Pi {}

impl Deref for Pi {
    type Target = PiRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Peripheral interface register block.
#[repr(C)]
pub struct PiRegisters {
    /// 0x00 - DRAM address
    pub pi_dram_addr_reg: PiDramAddrReg,

    /// 0x04 - PBUS (cartridge) address
    pub pi_cart_addr_reg: PiCartAddrReg,

    /// 0x08 - Read length
    pub pi_rd_len_reg: PiRdLenReg,

    /// 0x0C - Write length
    pub pi_wr_len_reg: PiWrLenReg,

    /// 0x10 - Status
    pub pi_status_reg: PiStatusReg,

    /// 0x14 - Domain 1 latency
    pub pi_bsd_dom1_lat_reg: PiBsdDom1LatReg,

    /// 0x18 - Domain 1 pulse width
    pub pi_bsd_dom1_pwd_reg: PiBsdDom1PwdReg,

    /// 0x1C - Domain 1 page size
    pub pi_bsd_dom1_pgs_reg: PiBsdDom1PgsReg,

    /// 0x20 - Domain 1 release
    pub pi_bsd_dom1_rls_reg: PiBsdDom1RlsReg,

    /// 0x24 - Domain 2 latency
    pub pi_bsd_dom2_lat_reg: PiBsdDom2LatReg,

    /// 0x28 - Domain 2 pulse width
    pub pi_bsd_dom2_pwd_reg: PiBsdDom2PwdReg,

    /// 0x2C - Domain 2 page size
    pub pi_bsd_dom2_pgs_reg: PiBsdDom2PgsReg,

    /// 0x30 - Domain 2 release
    pub pi_bsd_dom2_rls_reg: PiBsdDom2RlsReg,
}

bitfield! {
    /// Peripheral interface DRAM address register.
    pub struct PiDramAddrReg(pub u32): Debug {
        pub starting_rdram_address: u32 @ 0..24,
    }
}

bitfield! {
    /// Peripheral interface PBUS (cartridge) address register.
    pub struct PiCartAddrReg(pub u32): Debug {
        pub starting_ad16_address: u32 @ 0..32,
    }
}

bitfield! {
    /// Peripheral interface read length register.
    pub struct PiRdLenReg(pub u32): Debug {
        pub read_data_length: u32 @ 0..24,
    }
}

bitfield! {
    /// Peripheral interface write length register.
    pub struct PiWrLenReg(pub u32): Debug {
        pub write_data_length: u32 @ 0..24,
    }
}

bitfield! {
    /// Peripheral interface status register.
    pub struct PiStatusReg(pub u32): Debug {
        pub dma_busy: bool [read_only] @ 0,
        pub io_busy: bool [read_only] @ 1,
        pub error: bool [read_only] @ 2,

        pub reset_controller: bool [write_only] @ 0,
        pub clear_intr: bool [write_only] @ 1,
    }
}

bitfield! {
    /// Peripheral interface domain 1 latency register.
    pub struct PiBsdDom1LatReg(pub u32): Debug {
        pub latency: u8 @ 0..8,
    }
}

bitfield! {
    /// Peripheral interface domain 1 pulse width register.
    pub struct PiBsdDom1PwdReg(pub u32): Debug {
        pub pulse_width: u8 @ 0..8,
    }
}

bitfield! {
    /// Peripheral interface domain 1 page size register.
    pub struct PiBsdDom1PgsReg(pub u32): Debug {
        pub page_size: u8 @ 0..4,
    }
}

bitfield! {
    /// Peripheral interface domain 1 release register.
    pub struct PiBsdDom1RlsReg(pub u32): Debug {
        pub release: u8 @ 0..2,
    }
}

bitfield! {
    /// Peripheral interface domain 2 latency register.
    pub struct PiBsdDom2LatReg(pub u32): Debug {
        pub latency: u8 @ 0..8,
    }
}

bitfield! {
    /// Peripheral interface domain 2 pulse width register.
    pub struct PiBsdDom2PwdReg(pub u32): Debug {
        pub pulse_width: u8 @ 0..8,
    }
}

bitfield! {
    /// Peripheral interface domain 2 page size register.
    pub struct PiBsdDom2PgsReg(pub u32): Debug {
        pub page_size: u8 @ 0..4,
    }
}

bitfield! {
    /// Peripheral interface domain 2 release register.
    pub struct PiBsdDom2RlsReg(pub u32): Debug {
        pub release: u8 @ 0..2,
    }
}
