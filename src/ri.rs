//! # RDRAM interface (RI)

use proc_bitfield::bitfield;

use crate::{fields, registers};

/// # RI base address
pub const RI_BASE_ADDR: u32 = 0x0470_0000;

registers! {
    /// # RDRAM interface (RI)
    RI_BASE_ADDR => Ri {
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
}

bitfield! {
    /// # RI mode register
    pub struct RiModeReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub operating_mode: u8 [OperatingMode] @ 0..2,
        pub stop_transmit_active: bool @ 2,
        pub stop_receive_active: bool @ 3,
    }
}

bitfield! {
    /// # RI config register
    pub struct RiConfigReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub current_control_input: u8 [ControlInput] @ 0..6,
        pub current_control_enable: bool @ 6,
    }
}

bitfield! {
    /// # RI current load register
    pub struct RiCurrentLoadReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

bitfield! {
    /// # RI select register
    pub struct RiSelectReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub transmit_select: u8 [SignalTimings] @ 0..4,
        pub receive_select: u8 [SignalTimings] @ 4..8,
    }
}

bitfield! {
    /// # RI refresh register
    pub struct RiRefreshReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub clean_refresh_delay: u8 [RefreshDelay] @ 0..7,
        pub dirty_refresh_delay: u8 [RefreshDelay] @ 8..15,
        pub refresh_bank: bool @ 16,
        pub refresh_enable: bool @ 17,
        pub refresh_optimize: bool @ 18,
    }
}

bitfield! {
    /// # RI latency register
    pub struct RiLatencyReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_latency_overlap: u8 [DmaLatencyOverlap] @ 0..4,
    }
}

bitfield! {
    /// # RI read error register
    pub struct RiRerrorReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub nack_error: bool [read_only] @ 0,
        pub ack_error: bool [read_only] @ 1,
    }
}

bitfield! {
    /// # RI write error register
    pub struct RiWerrorReg(pub u32): Debug {
        pub raw: u32 @ ..,
    }
}

fields! [
    /// # Control input
    ux::u6 => ControlInput,

    /// # DMA latency/overlap
    ux::u4 => DmaLatencyOverlap,

    /// # Operating mode
    ux::u2 => OperatingMode,

    /// # Refresh delay
    ux::u7 => RefreshDelay,

    /// # Signal timings
    ux::u4 => SignalTimings,
];
