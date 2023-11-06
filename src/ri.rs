//! # RDRAM interface (RI)

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # RI base address
pub const RI_BASE_ADDR: u32 = 0x0470_0000;

/// # RDRAM interface (RI)
pub struct Ri;

impl_interface!(Ri, RiRegisters, RI_BASE_ADDR);

/// # RI register block
#[repr(C)]
pub struct RiRegisters {
    /// Mode
    pub ri_mode_reg: RiModeReg,

    /// Config
    pub ri_config_reg: RiConfigReg,

    /// Current load
    pub ri_current_load_reg: RiCurrentLoadReg,

    /// Select
    pub ri_select_reg: RiSelectReg,

    /// Refresh
    pub ri_refresh_reg: RiRefreshReg,

    /// Latency
    pub ri_latency_reg: RiLatencyReg,

    /// Read error
    pub ri_rerror_reg: RiRerrorReg,

    /// Write error
    pub ri_werror_reg: RiWerrorReg,
}

bitfield! {
    /// # # RI Mode Registe
    pub struct RiModeReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub operating_mode: u8 [get OperatingMode, try_set OperatingMode] @ 0..2,
        pub stop_transmit_active: bool @ 2,
        pub stop_receive_active: bool @ 3,
    }
}

bitfield! {
    /// # # RI Config Registe
    pub struct RiConfigReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub current_control_input: u8 [get ControlInput, try_set ControlInput] @ 0..6,
        pub current_control_enable: bool @ 6,
    }
}

bitfield! {
    /// # # RI Current Load Registe
    pub struct RiCurrentLoadReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RI select register
    pub struct RiSelectReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RI refresh register
    pub struct RiRefreshReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RI latency register
    pub struct RiLatencyReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RI read error register
    pub struct RiRerrorReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RI write error register
    pub struct RiWerrorReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

/// # Operating mode
#[derive(Debug)]
pub struct OperatingMode(pub u8);

impl_deref!(OperatingMode, u8);
impl_get!(OperatingMode, u8);
impl_set!(OperatingMode, u8, 0..2);

/// # Control input
#[derive(Debug)]
pub struct ControlInput(pub u8);

impl_deref!(ControlInput, u8);
impl_get!(ControlInput, u8);
impl_set!(ControlInput, u8, 0..6);
