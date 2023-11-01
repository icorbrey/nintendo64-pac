//! # RDRAM

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::impl_interface;

/// # RDRAM
pub struct Rdram;

impl_interface!(Rdram, RdramRegisters, 0x03F0_0000);

/// # RDRAM Register Block
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
    /// # RDRAM Device Type Register
    pub struct RdramDeviceTypeReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Device ID Register
    pub struct RdramDeviceIdReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Delay Register
    pub struct RdramDelayReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Mode Register
    pub struct RdramModeReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Ref Interval Register
    pub struct RdramRefIntervalReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Ref Row Register
    pub struct RdramRefRowReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Ras Interval Register
    pub struct RdramRasIntervalReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Min Interval Register
    pub struct RdramMinIntervalReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Address Select Register
    pub struct RdramAddrSelectReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM Device Manufacturer Register
    pub struct RdramDeviceManufReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}
