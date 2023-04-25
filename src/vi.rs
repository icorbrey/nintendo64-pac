//! # Video Interface Wrapper
//!
//! This module wraps the Nintendo 64's video interface registers and provides
//! type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use crate::HARDWARE;

/// The static address of the Nintendo 64's video interface registers.
#[cfg(target_vendor = "nintendo64")]
const VI_REGS_BASE: usize = 0x0440_0000;

#[cfg(not(target_vendor = "nintendo64"))]
lazy_static::lazy_static! {
    /// A registry access analogue for development and testing.
    ///
    /// We have to modify the registry access mechanism when building for
    /// architectures other than the Nintendo 64 since the production registry
    /// access mechanism accesses a static memory location. This is disallowed
    /// on modern operating systems, so we instead dynamically allocate the
    /// memory so that testing and development can occur.
    static ref REGISTERS: VideoInterfaceRegisters = unsafe { std::mem::zeroed() };
}

#[non_exhaustive]
pub struct VideoInterface;

impl VideoInterface {
    /// Gets a reference to the video interface registers.
    #[cfg(target_vendor = "nintendo64")]
    fn registers<'a>(&self) -> &'a VideoInterfaceRegisters {
        unsafe { &mut *(VI_REGS_BASE as *mut VideoInterfaceRegisters) }
    }

    /// Gets a reference to the video interface registers.
    #[cfg(not(target_vendor = "nintendo64"))]
    fn registers<'a>(&self) -> &'a REGISTERS {
        &REGISTERS
    }

    /// Returns ownership of the video interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.video_interface.drop(self) }
    }

    /// Clears an existing interrupt.
    pub fn clear_interrupt(&self) -> &Self {
        let registers = self.registers();
        let current_line = registers.current_halfline.read(HalfLine::HALF_LINE);
        self.registers()
            .current_halfline
            .write(HalfLine::HALF_LINE.val(current_line));
        self
    }
}

// This is a hack to allow code to run for development.
#[cfg(not(target_vendor = "nintendo64"))]
unsafe impl Sync for VideoInterfaceRegisters {}

register_structs! {
    VideoInterfaceRegisters {
        (0x0000 => pub control: ReadWrite<u32, Control::Register>),
        (0x0004 => pub framebuffer_address: ReadWrite<u32, DramAddress::Register>),
        (0x0008 => pub screen_width: ReadWrite<u32, LineWidth::Register>),
        (0x000C => pub interrupt_halfline: ReadWrite<u32, HalfLine::Register>),
        (0x0010 => pub current_halfline: ReadWrite<u32, HalfLine::Register>),
        (0x0014 => pub timing: ReadWrite<u32, Timing::Register>),
        (0x0018 => pub vertical_sync: ReadWrite<u32, HalfLine::Register>),
        (0x001C => pub horizontal_sync: ReadWrite<u32, LineWidth::Register>),
        (0x0020 => pub horizontal_sync_2: ReadWrite<u32, Leap::Register>),
        (0x0024 => pub horizontal_range: ReadWrite<u32, ScreenRange::Register>),
        (0x0028 => pub vertical_range: ReadWrite<u32, ScreenRange::Register>),
        (0x002C => pub color_burst: ReadWrite<u32, ScreenRange::Register>),
        (0x0030 => pub horizontal_scale: ReadWrite<u32, ScreenScale::Register>),
        (0x0034 => pub vertical_scale: ReadWrite<u32, ScreenScale::Register>),
        (0x0038 => @END),
    }
}

register_bitfields! {
    u32,

    Control [
        COLOR_DEPTH            OFFSET(0) NUMBITS(2)  [
            None = 0,
            FullColor = 2,
            DeepColor = 3,
        ],
        GAMMA_DITHER_ENABLE    OFFSET(2) NUMBITS(1)  [],
        GAMMA_ENABLE           OFFSET(3) NUMBITS(1)  [],
        DIVOT_ENABLE           OFFSET(4) NUMBITS(1)  [],
        SERRATE                OFFSET(6) NUMBITS(1)  [],
        ANTIALIAS_MODE         OFFSET(8) NUMBITS(2)  [
            Enabled = 0,
            EnabledAsNeeded = 1,
            ResamplingOnly = 2,
            Disabled = 3,
        ],
    ],

    DramAddress [
        ADDRESS                OFFSET(0)  NUMBITS(24) [],
    ],

    LineWidth [
        WIDTH                  OFFSET(0)  NUMBITS(12) [],
        LEAP_PATTERN           OFFSET(16) NUMBITS(5)  [],
    ],

    HalfLine [
        HALF_LINE              OFFSET(0)  NUMBITS(10) [],
    ],

    Timing [
        HORIZONTAL_SYNC_WIDTH  OFFSET(0)  NUMBITS(8)  [],
        COLOR_BURST_WIDTH      OFFSET(8)  NUMBITS(8)  [],
        VERTICAL_SYNC_WIDTH    OFFSET(16) NUMBITS(8)  [],
        COLOR_BURST_START      OFFSET(24) NUMBITS(8)  [],
    ],

    Leap [
        HORIZONTAL_SYNC_PERIOD OFFSET(0)  NUMBITS(12) [],
    ],

    ScreenRange [
        END_ACTIVE_VIDEO       OFFSET(0)  NUMBITS(10) [],
        START_ACTIVE_VIDEO     OFFSET(16) NUMBITS(10) [],
    ],

    ScreenScale [
        INVERSE_SCALE_FACTOR   OFFSET(0)  NUMBITS(12) [],
        SUBPIXEL_OFFSET        OFFSET(16) NUMBITS(12) [],
    ]
}
