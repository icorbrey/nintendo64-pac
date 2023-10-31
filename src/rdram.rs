//! # RDRAM

use core::ops::Deref;

use proc_bitfield::bitfield;

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
    pub rdram_device_type_reg: RdramDeviceTypeReg,

    /// `0x04` - Device ID
    pub rdram_device_id_reg: RdramDeviceIdReg,

    /// `0x08` - Delay
    pub rdram_delay_reg: RdramDelayReg,

    /// `0x0C` - Mode
    pub rdram_mode_reg: RdramModeReg,

    /// `0x10` - Ref interval
    pub rdram_ref_interval_reg: RdramRefIntervalReg,

    /// `0x14` - Ref row
    pub rdram_ref_row_reg: RdramRefRowReg,

    /// `0x18` - Ras interval
    pub rdram_ras_interval_reg: RdramRasIntervalReg,

    /// `0x1C` - Minimum interval
    pub rdram_min_interval_reg: RdramMinIntervalReg,

    /// `0x20` - Address select
    pub rdram_addr_select_reg: RdramAddrSelectReg,

    /// `0x24` - Device manufacturer
    pub rdram_device_manuf_reg: RdramDeviceManufReg,
}

bitfield! {
    /// RDRAM device type register.
    pub struct RdramDeviceTypeReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM device ID register.
    pub struct RdramDeviceIdReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM delay register.
    pub struct RdramDelayReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM mode register.
    pub struct RdramModeReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM ref interval register.
    pub struct RdramRefIntervalReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM ref row register.
    pub struct RdramRefRowReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM ras interval register.
    pub struct RdramRasIntervalReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM min interval register.
    pub struct RdramMinIntervalReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM address select register.
    pub struct RdramAddrSelectReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM device manufacturer register.
    pub struct RdramDeviceManufReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}
