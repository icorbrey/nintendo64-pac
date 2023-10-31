//! # DP Span

use core::ops::Deref;

use proc_bitfield::bitfield;

const DPS_BASE_REG: u32 = 0x0420_0000;

/// DP span.
pub struct Dps;

impl Dps {
    pub fn ptr() -> *const DpsRegisters {
        DPS_BASE_REG as *const _
    }
}

unsafe impl Sync for Dps {}

impl Deref for Dps {
    type Target = DpsRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// DP span register block.
#[repr(C)]
pub struct DpsRegisters {
    /// 0x00 - TBIST
    pub dps_tbist_reg: DpsTbistReg,

    /// 0x04 - Test mode
    pub dps_test_mode_reg: DpsTestModeReg,

    /// 0x08 - Buffer test address
    pub dps_buftest_addr_reg: DpsBuftestAddrReg,

    /// 0x0C - Buffer test data
    pub dps_buftest_data_reg: DpsBuftestDataReg,
}

bitfield! {
    /// DP span TBIST register.
    pub struct DpsTbistReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub bist_check: bool @ 0,
        pub bist_go: bool @ 1,
        pub bist_clear: bool [write_only] @ 2,
        pub bist_done: bool [read_only] @ 2,
        pub bist_fail: u8 [read_only] @ 3..11,
    }
}

bitfield! {
    /// DP span test mode register.
    pub struct DpsTestModeReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub span_buffer_test_access_enable: bool @ 0,
    }
}

bitfield! {
    /// DP span buffer test address register.
    pub struct DpsBuftestAddrReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub span_buffer_address: u8 @ 0..7,
    }
}

bitfield! {
    /// DP span buffer test data register.
    pub struct DpsBuftestDataReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub span_buffer_data: u32 @ 0..32,
    }
}
