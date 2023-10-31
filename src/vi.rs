//! # Video Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

const VI_BASE_REG: u32 = 0x0440_0000;

/// Video interface.
pub struct Vi;

impl Vi {
    pub fn ptr() -> *const ViRegisters {
        VI_BASE_REG as *const _
    }
}

unsafe impl Send for Vi {}

impl Deref for Vi {
    type Target = ViRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Video interface register block.
#[repr(C)]
pub struct ViRegisters {
    /// 0x00 - Status
    pub vi_status_reg: ViStatusReg,

    /// 0x04 - Frame buffer origin
    pub vi_origin_reg: ViOriginReg,

    /// 0x08 - Frame buffer line width
    pub vi_width_reg: ViWidthReg,

    /// 0x0C - Vertical interrupt
    pub vi_intr_reg: ViIntrReg,

    /// 0x10 - Current vertical line
    pub vi_current_reg: ViCurrentReg,

    /// 0x14 - Timing
    pub vi_timing_reg: ViTimingReg,

    /// 0x18 - Vertical sync
    pub vi_v_sync_reg: ViVSyncReg,

    /// 0x1C - Horizontal sync
    pub vi_h_sync_reg: ViHSyncReg,

    /// 0x20 - Horizontal sync leap
    pub vi_h_sync_leap_reg: ViHSyncLeapReg,

    /// 0x24 - Horizontal video
    pub vi_h_video_reg: ViHVideoReg,

    /// 0x28 - Vertical video
    pub vi_v_video_reg: ViVVideoReg,

    /// 0x2C - Vertical burst
    pub vi_v_burst_reg: ViVBurstReg,

    /// 0x30 - X-scale
    pub vi_x_scale_reg: ViXScaleReg,

    /// 0x34 - Y-scale
    pub vi_y_scale_reg: ViYScaleReg,
}

bitfield! {
    /// Video interface status register.
    pub struct ViStatusReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub pixel_size: u8 @ 0..2,
        pub gamma_dither_enable: bool @ 2,
        pub gamma_enable: bool @ 3,
        pub divot_enable: bool @ 4,
        pub serrate: bool @ 6,
        pub antialias_mode: u8 @ 8..10,
    }
}

bitfield! {
    /// Video interface frame buffer origin register.
    pub struct ViOriginReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub frame_buffer_origin: u32 @ 0..24,
    }
}

bitfield! {
    /// Video interface frame buffer line width register.
    pub struct ViWidthReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub frame_buffer_line_width: u16 @ 0..12,
    }
}

bitfield! {
    /// Video interface vertical interrupt register.
    pub struct ViIntrReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub intr_half_line: u16 @ 0..10,
    }
}

bitfield! {
    /// Video interface current vertical line register.
    pub struct ViCurrentReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub current_half_line: u16 @ 0..10,
    }
}

bitfield! {
    /// Video interface timing register.
    pub struct ViTimingReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub horizontal_sync_width: u8 @ 0..8,
        pub color_burst_width: u8 @ 8..16,
        pub vertical_sync_width: u8 @ 16..20,
        pub color_burst_offset: u16 @ 20..30,
    }
}

bitfield! {
    /// Video interface vertical sync register.
    pub struct ViVSyncReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub half_lines_per_field: u16 @ 0..10,
    }
}

bitfield! {
    /// Video interface horizontal sync register.
    pub struct ViHSyncReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub line_duration: u16 @ 0..12,
        pub leap_pattern: u8 @ 16..21,
    }
}

bitfield! {
    /// Video interface horizontal sync leap register.
    pub struct ViHSyncLeapReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub h_sync_period_0: u16 @ 0..12,
        pub h_sync_period_1: u16 @ 16..28,
    }
}

bitfield! {
    /// Video interface horizontal video register.
    pub struct ViHVideoReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub end_active_video: u16 @ 0..10,
        pub start_active_video: u16 @ 16..26,
    }
}

bitfield! {
    /// Video interface vertical video register.
    pub struct ViVVideoReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub end_active_video: u16 @ 0..10,
        pub start_active_video: u16 @ 16..26,
    }
}

bitfield! {
    /// Video interface vertical burst register.
    pub struct ViVBurstReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub end_color_burst: u16 @ 0..10,
        pub start_color_burst: u16 @ 16..26,
    }
}

bitfield! {
    /// Video interface X-scale register.
    pub struct ViXScaleReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub inverse_scale_factor: u16 @ 0..12,
        pub subpixel_offset: u16 @ 16..28,
    }
}

bitfield! {
    /// Video interface Y-scale register.
    pub struct ViYScaleReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub inverse_scale_factor: u16 @ 0..12,
        pub subpixel_offset: u16 @ 16..28,
    }
}
