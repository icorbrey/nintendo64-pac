//! # Video interface (VI)

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{enums, fields, interface};

/// # VI base address
pub const VI_BASE_ADDR: u32 = 0x0440_0000;

/// # Video interface (VI)
pub struct Vi;

interface!(Vi, ViRegisters, VI_BASE_ADDR);

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
        pub pixel_size: u8 [PixelSize] @ 0..2,
        pub gamma_dither_enable: bool @ 2,
        pub gamma_enable: bool @ 3,
        pub divot_enable: bool @ 4,
        pub serrate: bool @ 6,
        pub antialias_mode: u8 [AntialiasMode] @ 8..10,
    }
}

bitfield! {
    /// # VI frame buffer origin register
    pub struct ViOriginReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub frame_buffer_origin: u32 [RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # VI frame buffer line width register
    pub struct ViWidthReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub frame_buffer_line_width: u16 [LineWidth] @ 0..12,
    }
}

bitfield! {
    /// # VI vertical interrupt register
    pub struct ViIntrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub intr_half_line: u16 [HalflineIndex] @ 0..10,
    }
}

bitfield! {
    /// # VI current vertical line register
    pub struct ViCurrentReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub current_half_line: u16 [HalflineIndex] @ 0..10,
    }
}

bitfield! {
    /// # VI timing register
    pub struct ViTimingReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub horizontal_sync_width: u8 [PixelWidth] @ 0..8,
        pub color_burst_width: u8 [PixelWidth] @ 8..16,
        pub vertical_sync_width: u8 [HalflineHeight] @ 16..20,
        pub color_burst_offset: u16 [HalflineIndex] @ 20..30,
    }
}

bitfield! {
    /// # VI vertical sync register
    pub struct ViVSyncReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub half_lines_per_field: u16 [HalflineIndex] @ 0..10,
    }
}

bitfield! {
    /// # VI horizontal sync register
    pub struct ViHSyncReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub line_duration: u16 [LineDuration] @ 0..12,
        pub leap_pattern: u8 [LeapPattern] @ 16..21,
    }
}

bitfield! {
    /// # VI horizontal sync leap register
    pub struct ViHSyncLeapReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub h_sync_period_0: u16 [LineDuration] @ 0..12,
        pub h_sync_period_1: u16 [LineDuration] @ 16..28,
    }
}

bitfield! {
    /// # VI horizontal video register
    pub struct ViHVideoReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub end_active_video: u16 [PixelIndex] @ 0..10,
        pub start_active_video: u16 [PixelIndex] @ 16..26,
    }
}

bitfield! {
    /// # VI vertical video register
    pub struct ViVVideoReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub end_active_video: u16 [HalflineIndex] @ 0..10,
        pub start_active_video: u16 [HalflineIndex] @ 16..26,
    }
}

bitfield! {
    /// # VI vertical burst register
    pub struct ViVBurstReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub end_color_burst: u16 [HalflineIndex] @ 0..10,
        pub start_color_burst: u16 [HalflineIndex] @ 16..26,
    }
}

bitfield! {
    /// # VI X-scale register
    pub struct ViXScaleReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub inverse_scale_factor: u16 [InverseScaleFactor] @ 0..12,
        pub subpixel_offset: u16 [SubpixelOffset] @ 16..28,
    }
}

bitfield! {
    /// # VI Y-scale register
    pub struct ViYScaleReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub inverse_scale_factor: u16 [InverseScaleFactor] @ 0..12,
        pub subpixel_offset: u16 [SubpixelOffset] @ 16..28,
    }
}

fields! [
    /// # Line width
    ux::u12 => LineWidth,

    /// # Halfline Index
    ux::u10 => HalflineIndex,

    /// # Halfline height
    ux::u4 => HalflineHeight,

    /// # Inverse scale factor
    ux::u12 => InverseScaleFactor,

    /// # Leap pattern
    ux::u5 => LeapPattern,

    /// # Line duration
    ux::u12 => LineDuration,

    /// # Pixel Index
    ux::u10 => PixelIndex,

    /// # Pixel width
    u8 => PixelWidth,

    /// # RDRAM address
    ux::u24 => RdramAddress,

    /// # Subpixel offset
    ux::u12 => SubpixelOffset,
];

enums! [
    /// # Pixel size
    u8 => PixelSize {
        0 => Blank,
        2 => SixteenBit,
        3 => ThirtyTwoBit,
    },

    /// # Antialias mode
    u8 => AntialiasMode {
        0 => Full,
        1 => Optimized,
        2 => ResampleOnly,
        3 => None,
    },
];
