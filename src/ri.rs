//! # RDRAM Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

const RI_BASE_REG: u32 = 0x0470_0000;

/// RDRAM interface.
pub struct Ri;

impl Ri {
    pub fn ptr() -> *const RiRegisters {
        RI_BASE_REG as *const _
    }
}

unsafe impl Sync for Ri {}

impl Deref for Ri {
    type Target = RiRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// RDRAM interface register block.
#[repr(C)]
pub struct RiRegisters {
    /// `0x00` - Mode
    pub ri_mode_reg: RiModeReg,

    /// `0x04` - Config
    pub ri_config_reg: RiConfigReg,

    /// `0x08` - Current load
    pub ri_current_load_reg: RiCurrentLoadReg,

    /// `0x0C` - Select
    pub ri_select_reg: RiSelectReg,

    /// `0x10` - Refresh
    pub ri_refresh_reg: RiRefreshReg,

    /// `0x14` - Latency
    pub ri_latency_reg: RiLatencyReg,

    /// `0x18` - Read error
    pub ri_rerror_reg: RiRerrorReg,

    /// `0x1C` - Write error
    pub ri_werror_reg: RiWerrorReg,
}

bitfield! {
    /// RDRAM interface mode register.
    pub struct RiModeReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface config register.
    pub struct RiConfigReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface current load register.
    pub struct RiCurrentLoadReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface select register.
    pub struct RiSelectReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface refresh register.
    pub struct RiRefreshReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface latency register.
    pub struct RiLatencyReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface read error register.
    pub struct RiRerrorReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface write error register.
    pub struct RiWerrorReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,
    }
}
