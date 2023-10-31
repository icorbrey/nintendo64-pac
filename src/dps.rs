//! # DP Span

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # DP Span
pub struct Dps;

impl_interface!(Dps, DpsRegisters, 0x0420_0000);

/// # DP Span Register Block
#[repr(C)]
pub struct DpsRegisters {
    /// `0x00` - TBIST
    pub dps_tbist_reg: DpsTbistReg,

    /// `0x04` - Test mode
    pub dps_test_mode_reg: DpsTestModeReg,

    /// `0x08` - Buffer test address
    pub dps_buftest_addr_reg: DpsBuftestAddrReg,

    /// `0x0C` - Buffer test data
    pub dps_buftest_data_reg: DpsBuftestDataReg,
}

bitfield! {
    /// # DP Span TBIST Register
    pub struct DpsTbistReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub bist_check: bool @ 0,
        pub bist_go: bool @ 1,
        pub bist_clear: bool [write_only] @ 2,
        pub bist_done: bool [read_only] @ 2,
        pub bist_fail: u8 [read_only, get BistFail] @ 3..11,
    }
}

bitfield! {
    /// # DP Span Test Mode Register
    pub struct DpsTestModeReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub span_buffer_test_access_enable: bool @ 0,
    }
}

bitfield! {
    /// # DP Span Buffer Test Address Register
    pub struct DpsBuftestAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub span_buffer_address: u8 [get BufferTestAddress, try_set BufferTestAddress] @ 0..7,
    }
}

bitfield! {
    /// # DP Span Buffer Test Data Register
    pub struct DpsBuftestDataReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub span_buffer_data: u32 [get BufferTestData, try_set BufferTestData] @ 0..32,
    }
}

/// # BIST Failure
#[derive(Debug)]
pub struct BistFail(pub u8);

impl_deref!(BistFail, u8);
impl_get!(BistFail, u8);

/// # Buffer Test Address
#[derive(Debug)]
pub struct BufferTestAddress(pub u8);

impl_deref!(BufferTestAddress, u8);
impl_get!(BufferTestAddress, u8);
impl_set!(BufferTestAddress, u8, 0..7);

/// # Buffer Test Data
#[derive(Debug)]
pub struct BufferTestData(pub u32);

impl_deref!(BufferTestData, u32);
impl_get!(BufferTestData, u32);
impl_set!(BufferTestData, u32, 0..32);
