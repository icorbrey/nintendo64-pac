//! # Peripheral interface (PI)

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

pub const PI_BASE_ADDR: u32 = 0x0460_0000;

/// # Peripheral interface (PI)
pub struct Pi;

impl_interface!(Pi, PiRegisters, PI_BASE_ADDR);

/// # PI register block
#[repr(C)]
pub struct PiRegisters {
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

bitfield! {
    /// # PI DRAM address register
    pub struct PiDramAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_rdram_address: u32 [get RdramAddress, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # PI PBUS (cartridge) address register
    pub struct PiCartAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_ad16_address: u32 [get Ad16Address, try_set Ad16Address] @ 0..32,
    }
}

bitfield! {
    /// # PI read length register
    pub struct PiRdLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub read_data_length: u32 [get DataLength, try_set DataLength] @ 0..24,
    }
}

bitfield! {
    /// # PI write length register
    pub struct PiWrLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub write_data_length: u32 [get DataLength, try_set DataLength] @ 0..24,
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
        pub latency: u8 [get Latency, try_set Latency] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 1 pulse width register
    pub struct PiBsdDom1PwdReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub pulse_width: u8 [get PulseWidth, try_set PulseWidth] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 1 page size register
    pub struct PiBsdDom1PgsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub page_size: u8 [get PageSize, try_set PageSize] @ 0..4,
    }
}

bitfield! {
    /// # PI domain 1 release register
    pub struct PiBsdDom1RlsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub release: u8 [get Release, try_set Release] @ 0..2,
    }
}

bitfield! {
    /// # PI domain 2 latency register
    pub struct PiBsdDom2LatReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub latency: u8 [get Latency, try_set Latency] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 2 pulse width register
    pub struct PiBsdDom2PwdReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub pulse_width: u8 [get PulseWidth, try_set PulseWidth] @ 0..8,
    }
}

bitfield! {
    /// # PI domain 2 page size register
    pub struct PiBsdDom2PgsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub page_size: u8 [get PageSize, try_set PageSize] @ 0..4,
    }
}

bitfield! {
    /// # PI domain 2 release register
    pub struct PiBsdDom2RlsReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub release: u8 [get Release, try_set Release] @ 0..2,
    }
}

/// # RDRAM Address
#[derive(Debug)]
pub struct RdramAddress(pub u32);

impl_deref!(RdramAddress, u32);
impl_get!(RdramAddress, u32);
impl_set!(RdramAddress, u32, 0..24);

/// # AD16 Address
#[derive(Debug)]
pub struct Ad16Address(pub u32);

impl_deref!(Ad16Address, u32);
impl_get!(Ad16Address, u32);
impl_set!(Ad16Address, u32, 0..32);

/// # Data Length
#[derive(Debug)]
pub struct DataLength(pub u32);

impl_deref!(DataLength, u32);
impl_get!(DataLength, u32);
impl_set!(DataLength, u32, 0..24);

/// # Latency
#[derive(Debug)]
pub struct Latency(pub u8);

impl_deref!(Latency, u8);
impl_get!(Latency, u8);
impl_set!(Latency, u8, 0..8);

/// # Pulse Width
#[derive(Debug)]
pub struct PulseWidth(pub u8);

impl_deref!(PulseWidth, u8);
impl_get!(PulseWidth, u8);
impl_set!(PulseWidth, u8, 0..8);

/// # Page Size
#[derive(Debug)]
pub struct PageSize(pub u8);

impl_deref!(PageSize, u8);
impl_get!(PageSize, u8);
impl_set!(PageSize, u8, 0..4);

/// # Release
#[derive(Debug)]
pub struct Release(pub u8);

impl_deref!(Release, u8);
impl_get!(Release, u8);
impl_set!(Release, u8, 0..2);
