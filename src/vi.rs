use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

const VI_REGS_BASE: usize = 0x0440_0000;

pub struct VideoInterface;

impl VideoInterface {
    pub fn registers<'a>(&self) -> &'a VideoInterfaceRegisters {
        unsafe { &mut *(VI_REGS_BASE as *mut VideoInterfaceRegisters) }
    }
}

register_structs! {
    pub VideoInterfaceRegisters {
        (0x0000 => pub status: ReadWrite<u32, Status::Register>),
        (0x0004 => pub origin: ReadWrite<u32, DmaAddress::Register>),
        (0x0008 => pub width: ReadWrite<u32, LineWidth::Register>),
        (0x000C => pub vertical_interrupt: ReadWrite<u32, HalfLine::Register>),
        (0x0010 => pub vertical_current: ReadWrite<u32, HalfLine::Register>),
        (0x0014 => pub timing: ReadWrite<u32, Timing::Register>),
        (0x0018 => pub vertical_sync: ReadWrite<u32, HalfLine::Register>),
        (0x001C => pub horizontal_sync: ReadWrite<u32, LineWidth::Register>),
        (0x0020 => pub leap: ReadWrite<u32, Leap::Register>),
        (0x0024 => pub horizontal_video: ReadWrite<u32, ScreenRange::Register>),
        (0x0028 => pub vertical_video: ReadWrite<u32, ScreenRange::Register>),
        (0x002C => pub vertical_burst: ReadWrite<u32, ScreenRange::Register>),
        (0x0030 => pub x_scale: ReadWrite<u32, ScreenScale::Register>),
        (0x0034 => pub y_scale: ReadWrite<u32, ScreenScale::Register>),
        (0x0038 => @END),
    }
}

register_bitfields! {
    u32,

    pub Status [
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

    pub DmaAddress [
        ADDRESS                OFFSET(0)  NUMBITS(24) [],
    ],

    pub LineWidth [
        WIDTH                  OFFSET(0)  NUMBITS(12) [],
        LEAP_PATTERN           OFFSET(16) NUMBITS(5)  [],
    ],

    pub HalfLine [
        HALF_LINE              OFFSET(0)  NUMBITS(10) [],
    ],

    pub Timing [
        HORIZONTAL_SYNC_WIDTH  OFFSET(0)  NUMBITS(8)  [],
        COLOR_BURST_WIDTH      OFFSET(8)  NUMBITS(8)  [],
        VERTICAL_SYNC_WIDTH    OFFSET(16) NUMBITS(8)  [],
        COLOR_BURST_START      OFFSET(24) NUMBITS(8)  [],
    ],

    pub Leap [
        HORIZONTAL_SYNC_PERIOD OFFSET(0)  NUMBITS(12) [],
    ],

    pub ScreenRange [
        END_ACTIVE_VIDEO       OFFSET(0)  NUMBITS(10) [],
        START_ACTIVE_VIDEO     OFFSET(16) NUMBITS(10) [],
    ],

    pub ScreenScale [
        INVERSE_SCALE_FACTOR   OFFSET(0)  NUMBITS(12) [],
        SUBPIXEL_OFFSET        OFFSET(16) NUMBITS(12) [],
    ]
}
