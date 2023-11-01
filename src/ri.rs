//! # RDRAM Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # RDRAM Interface
pub struct Ri;

impl_interface!(Ri, RiRegisters, 0x0470_0000);

/// # RDRAM Interface Register Block
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
    /// # RDRAM Interface Mode Register
    pub struct RiModeReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub operating_mode: u8 [get OperatingMode, try_set OperatingMode] @ 0..2,
        pub stop_transmit_active: bool @ 2,
        pub stop_receive_active: bool @ 3,
    }
}

bitfield! {
    /// # RDRAM Interface Config Register
    pub struct RiConfigReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub current_control_input: u8 [get ControlInput, try_set ControlInput] @ 0..6,
        pub current_control_enable: bool @ 6,
    }
}

bitfield! {
    /// # RDRAM Interface Current Load Register
    pub struct RiCurrentLoadReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface select register.
    pub struct RiSelectReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface refresh register.
    pub struct RiRefreshReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface latency register.
    pub struct RiLatencyReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface read error register.
    pub struct RiRerrorReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// RDRAM interface write error register.
    pub struct RiWerrorReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

/// # Operating Mode
#[derive(Debug)]
pub struct OperatingMode(pub u8);

impl_deref!(OperatingMode, u8);
impl_get!(OperatingMode, u8);
impl_set!(OperatingMode, u8, 0..2);

/// # Control Input
#[derive(Debug)]
pub struct ControlInput(pub u8);

impl_deref!(ControlInput, u8);
impl_get!(ControlInput, u8);
impl_set!(ControlInput, u8, 0..6);
