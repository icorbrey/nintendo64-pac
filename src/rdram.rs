//! # RDRAM

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::impl_interface;

/// # RDRAM
pub struct Rdram;

impl_interface!(Rdram, RdramRegisters, 0x03F0_0000);

/// # RDRAM register block
#[repr(C)]
pub struct RdramRegisters {
    /// Device type
    pub rdram_device_type_reg: RdramDeviceTypeReg,

    /// Device ID
    pub rdram_device_id_reg: RdramDeviceIdReg,

    /// Delay
    pub rdram_delay_reg: RdramDelayReg,

    /// Mode
    pub rdram_mode_reg: RdramModeReg,

    /// Ref interval
    pub rdram_ref_interval_reg: RdramRefIntervalReg,

    /// Ref row
    pub rdram_ref_row_reg: RdramRefRowReg,

    /// Ras interval
    pub rdram_ras_interval_reg: RdramRasIntervalReg,

    /// Minimum interval
    pub rdram_min_interval_reg: RdramMinIntervalReg,

    /// Address select
    pub rdram_addr_select_reg: RdramAddrSelectReg,

    /// Device manufacturer
    pub rdram_device_manuf_reg: RdramDeviceManufReg,
}

bitfield! {
    /// # RDRAM device type register
    pub struct RdramDeviceTypeReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM device ID register
    pub struct RdramDeviceIdReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM delay register
    pub struct RdramDelayReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM mode register
    pub struct RdramModeReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM ref interval register
    pub struct RdramRefIntervalReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM ref row register
    pub struct RdramRefRowReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM ras interval register
    pub struct RdramRasIntervalReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM min interval register
    pub struct RdramMinIntervalReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM address select register
    pub struct RdramAddrSelectReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RDRAM device manufacturer register
    pub struct RdramDeviceManufReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}
