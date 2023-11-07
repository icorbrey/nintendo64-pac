//! # Display processor span (DPS)

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{fields, interface};

/// # DPS base address
pub const DPS_BASE_ADDR: u32 = 0x0420_0000;

/// # Display processor span (DPS)
pub struct Dps;

interface!(Dps, DpsRegisters, DPS_BASE_ADDR);

/// # DPS Register Block
#[repr(C)]
pub struct DpsRegisters {
    /// TBIST
    pub dps_tbist_reg: DpsTbistReg,

    /// Test mode
    pub dps_test_mode_reg: DpsTestModeReg,

    /// Buffer test address
    pub dps_buftest_addr_reg: DpsBuftestAddrReg,

    /// Buffer test data
    pub dps_buftest_data_reg: DpsBuftestDataReg,
}

bitfield! {
    /// # `DPS_TBIST_REG`
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
    /// # `DPS_TEST_MODE_REG`
    pub struct DpsTestModeReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub span_buffer_test_access_enable: bool @ 0,
    }
}

bitfield! {
    /// # `DPS_BUFTEST_ADDR_REG`
    pub struct DpsBuftestAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub span_buffer_address: u8 [BufferTestAddress] @ 0..7,
    }
}

bitfield! {
    /// # `DPS_BUFTEST_DATA_REG`
    pub struct DpsBuftestDataReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub span_buffer_data: u32 [BufferTestData] @ 0..32,
    }
}

fields! [
    /// # BIST failure
    u8 => BistFail,

    /// # Buffer test address
    ux::u7 => BufferTestAddress,

    /// # Buffer test data
    u32 => BufferTestData,
];
