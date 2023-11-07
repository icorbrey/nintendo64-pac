//! # Peripheral interface (PI)

use proc_bitfield::bitfield;

use crate::{fields, registers};

/// # PI base address
pub const PI_BASE_ADDR: u32 = 0x0460_0000;

registers! {
    /// # Peripheral interface (PI)
    PI_BASE_ADDR => Pi {
        /// DRAM address
        pub pi_dram_addr_reg: PiDramAddrReg,

        /// PBUS (cartridge) address
        pub pi_cart_addr_reg: PiCartAddrReg,

        /// Read length
        pub pi_rd_len_reg: PiRdLenReg,

        /// Write length
        pub pi_wr_len_reg: PiWrLenReg,

        /// Status
        pub pi_status_reg: PiStatusReg,

        /// Domain 1 latency
        pub pi_bsd_dom1_lat_reg: PiBsdDom1LatReg,

        /// Domain 1 pulse width
        pub pi_bsd_dom1_pwd_reg: PiBsdDom1PwdReg,

        /// Domain 1 page size
        pub pi_bsd_dom1_pgs_reg: PiBsdDom1PgsReg,

        /// Domain 1 release
        pub pi_bsd_dom1_rls_reg: PiBsdDom1RlsReg,

        /// Domain 2 latency
        pub pi_bsd_dom2_lat_reg: PiBsdDom2LatReg,

        /// Domain 2 pulse width
        pub pi_bsd_dom2_pwd_reg: PiBsdDom2PwdReg,

        /// Domain 2 page size
        pub pi_bsd_dom2_pgs_reg: PiBsdDom2PgsReg,

        /// Domain 2 release
        pub pi_bsd_dom2_rls_reg: PiBsdDom2RlsReg,
    }
}

bitfield! {
    /// # PI DRAM address register
    pub struct PiDramAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_rdram_address: u32 [RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # PI PBUS (cartridge) address register
    pub struct PiCartAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_ad16_address: u32 [Ad16Address] @ 0..32,
    }
}

bitfield! {
    /// # PI read length register
    pub struct PiRdLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub read_data_length: u32 [DataLength] @ 0..24,
    }
}

bitfield! {
    /// # PI write length register
    pub struct PiWrLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub write_data_length: u32 [DataLength] @ 0..24,
    }
}

bitfield! {
    /// # PI status register
    pub struct PiStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_busy: bool [read_only] @ 0,
        pub io_busy: bool [read_only] @ 1,
        pub error: bool [read_only] @ 2,
        pub reset_controller: bool [write_only] @ 0,
        pub clear_intr: bool [write_only] @ 1,
    }
}

bitfield! {
    /// # PI domain 1 latency register
    pub struct PiBsdDom1LatReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub latency: u8 [Latency] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 1 pulse width register
    pub struct PiBsdDom1PwdReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub pulse_width: u8 [PulseWidth] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 1 page size register
    pub struct PiBsdDom1PgsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub page_size: u8 [PageSize] @ 0..4,
    }
}

bitfield! {
    /// # PI domain 1 release register
    pub struct PiBsdDom1RlsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub release: u8 [Release] @ 0..2,
    }
}

bitfield! {
    /// # PI domain 2 latency register
    pub struct PiBsdDom2LatReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub latency: u8 [Latency] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 2 pulse width register
    pub struct PiBsdDom2PwdReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub pulse_width: u8 [PulseWidth] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 2 page size register
    pub struct PiBsdDom2PgsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub page_size: u8 [PageSize] @ 0..4,
    }
}

bitfield! {
    /// # PI domain 2 release register
    pub struct PiBsdDom2RlsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub release: u8 [Release] @ 0..2,
    }
}

fields! [
    /// # AD16 address
    u32 => Ad16Address,

    /// # Data length
    ux::u24 => DataLength,

    /// # Latency
    u8 => Latency,

    /// # Page size
    ux::u4 => PageSize,

    /// # Pulse width
    u8 => PulseWidth,

    /// # RDRAM address
    ux::u24 => RdramAddress,

    /// # Release
    ux::u2 => Release,
];
