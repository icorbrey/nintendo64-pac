//! # Video interface (VI)

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_enum, impl_get, impl_interface, impl_set};

/// # VI base address
pub const VI_BASE_ADDR: u32 = 0x0440_0000;

/// # Video interface (VI)
pub struct Vi;

impl_interface!(Vi, ViRegisters, VI_BASE_ADDR);

/// # VI register block
#[repr(C)]
pub struct ViRegisters {
    /// Status
    pub vi_status_reg: ViStatusReg,

    /// Frame buffer origin
    pub vi_origin_reg: ViOriginReg,

    /// Frame buffer line width
    pub vi_width_reg: ViWidthReg,

    /// Vertical interrupt
    pub vi_intr_reg: ViIntrReg,

    /// Current vertical line
    pub vi_current_reg: ViCurrentReg,

    /// Timing
    pub vi_timing_reg: ViTimingReg,

    /// Vertical sync
    pub vi_v_sync_reg: ViVSyncReg,

    /// Horizontal sync
    pub vi_h_sync_reg: ViHSyncReg,

    /// Horizontal sync leap
    pub vi_h_sync_leap_reg: ViHSyncLeapReg,

    /// Horizontal video
    pub vi_h_video_reg: ViHVideoReg,

    /// Vertical video
    pub vi_v_video_reg: ViVVideoReg,

    /// Vertical burst
    pub vi_v_burst_reg: ViVBurstReg,

    /// X-scale
    pub vi_x_scale_reg: ViXScaleReg,

    /// Y-scale
    pub vi_y_scale_reg: ViYScaleReg,
}

bitfield! {
    /// # VI status register
    pub struct ViStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub pixel_size: u8 [get PixelSize, set PixelSize] @ 0..2,
        pub gamma_dither_enable: bool @ 2,
        pub gamma_enable: bool @ 3,
        pub divot_enable: bool @ 4,
        pub serrate: bool @ 6,
        pub antialias_mode: u8 [get AntialiasMode, set AntialiasMode] @ 8..10,
    }
}

bitfield! {
    /// # VI frame buffer origin register
    pub struct ViOriginReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub frame_buffer_origin: u32 [get RdramAddress, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # VI frame buffer line width register
    pub struct ViWidthReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub frame_buffer_line_width: u16 [get LineWidth, try_set LineWidth] @ 0..12,
    }
}

bitfield! {
    /// # VI vertical interrupt register
    pub struct ViIntrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub intr_half_line: u16 [get HalflineIndex, try_set HalflineIndex] @ 0..10,
    }
}

bitfield! {
    /// # VI current vertical line register
    pub struct ViCurrentReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub current_half_line: u16 @ 0..10,
    }
}

bitfield! {
    /// # VI timing register
    pub struct ViTimingReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub horizontal_sync_width: u8 @ 0..8,
        pub color_burst_width: u8 @ 8..16,
        pub vertical_sync_width: u8 @ 16..20,
        pub color_burst_offset: u16 @ 20..30,
    }
}

bitfield! {
    /// # VI vertical sync register
    pub struct ViVSyncReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub half_lines_per_field: u16 @ 0..10,
    }
}

bitfield! {
    /// # VI horizontal sync register
    pub struct ViHSyncReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub line_duration: u16 @ 0..12,
        pub leap_pattern: u8 @ 16..21,
    }
}

bitfield! {
    /// # VI horizontal sync leap register
    pub struct ViHSyncLeapReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub h_sync_period_0: u16 @ 0..12,
        pub h_sync_period_1: u16 @ 16..28,
    }
}

bitfield! {
    /// # VI horizontal video register
    pub struct ViHVideoReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub end_active_video: u16 @ 0..10,
        pub start_active_video: u16 @ 16..26,
    }
}

bitfield! {
    /// # VI vertical video register
    pub struct ViVVideoReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub end_active_video: u16 @ 0..10,
        pub start_active_video: u16 @ 16..26,
    }
}

bitfield! {
    /// # VI vertical burst register
    pub struct ViVBurstReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub end_color_burst: u16 @ 0..10,
        pub start_color_burst: u16 @ 16..26,
    }
}

bitfield! {
    /// # VI X-scale register
    pub struct ViXScaleReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub inverse_scale_factor: u16 @ 0..12,
        pub subpixel_offset: u16 @ 16..28,
    }
}

bitfield! {
    /// # VI Y-scale register
    pub struct ViYScaleReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub inverse_scale_factor: u16 @ 0..12,
        pub subpixel_offset: u16 @ 16..28,
    }
}

/// # Pixel size
#[derive(Debug)]
pub enum PixelSize {
    Blank,
    SixteenBit,
    ThirtyTwoBit,
}

impl_enum!(PixelSize, u8, {
    0 => PixelSize::Blank,
    2 => PixelSize::SixteenBit,
    3 => PixelSize::ThirtyTwoBit,
});

/// # Antialias mode
#[derive(Debug)]
pub enum AntialiasMode {
    Full,
    Optimized,
    ResampleOnly,
    None,
}

impl_enum!(AntialiasMode, u8, {
    0 => AntialiasMode::Full,
    1 => AntialiasMode::Optimized,
    2 => AntialiasMode::ResampleOnly,
    3 => AntialiasMode::None,
});

/// # RDRAM address
#[derive(Debug)]
pub struct RdramAddress(pub u32);

impl_deref!(RdramAddress, u32);
impl_get!(RdramAddress, u32);
impl_set!(RdramAddress, u32, 0..24);

/// # Line width
#[derive(Debug)]
pub struct LineWidth(pub u16);

impl_deref!(LineWidth, u16);
impl_get!(LineWidth, u16);
impl_set!(LineWidth, u16, 0..12);

/// # Halfline index
#[derive(Debug)]
pub struct HalflineIndex(pub u16);

impl_deref!(HalflineIndex, u16);
impl_get!(HalflineIndex, u16);
impl_set!(HalflineIndex, u16, 0..10);
