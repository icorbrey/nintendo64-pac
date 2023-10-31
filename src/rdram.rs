//! # RDRAM

use core::ops::Deref;

const RDRAM_BASE_REG: u32 = 0x03F0_0000;

/// RDRAM.
pub struct Rdram;

impl Rdram {
    pub fn ptr() -> *const RdramRegisters {
        RDRAM_BASE_REG as *const _
    }
}

unsafe impl Sync for Rdram {}

impl Deref for Rdram {
    type Target = RdramRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// RDRAM register block.
#[repr(C)]
pub struct RdramRegisters {
    /// `0x00` - Device type
    pub rdram_device_type_reg: u32,

    /// `0x04` - Device ID
    pub rdram_device_id_reg: u32,

    /// `0x08` - Delay
    pub rdram_delay_reg: u32,

    /// `0x0C` - Mode
    pub rdram_mode_reg: u32,

    /// `0x10` - Ref interval
    pub rdram_ref_interval_reg: u32,

    /// `0x14` - Ref row
    pub rdram_ref_row_reg: u32,

    /// `0x18` - Ras interval
    pub rdram_ras_interval_reg: u32,

    /// `0x1C` - Minimum interval
    pub rdram_min_interval_reg: u32,

    /// `0x20` - Address select
    pub rdram_addr_select_reg: u32,

    /// `0x24` - Device manufacturer
    pub rdram_device_manuf_reg: u32,
}
