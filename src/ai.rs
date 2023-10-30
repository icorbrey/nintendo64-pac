//! # Audio Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

const AI_BASE_REG: u32 = 0x0450_0000;

/// Audio interface.
pub struct Ai;

impl Ai {
    pub fn ptr() -> *const AiRegisters {
        AI_BASE_REG as *const _
    }
}

impl Deref for Ai {
    type Target = AiRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// Audio interface register block.
#[repr(C)]
pub struct AiRegisters {
    /// `0x00` - Address of audio sample in DRAM.
    pub ai_dram_addr_reg: AiDramAddrReg,

    /// `0x04` - Length of audio sample.
    pub ai_len_reg: AiLenReg,

    /// `0x08` - Control register #TK
    pub ai_control_reg: AiControlReg,

    /// `0x0C` - Status register #TK
    pub ai_status_reg: AiStatusReg,

    /// `0x10` - DAC rate register #TK
    pub ai_dacrate_reg: AiDacrateReg,

    /// `0x14` - Bitrate register #TK
    pub ai_bitrate_reg: AiBitrateReg,
}

bitfield! {
    /// Audio interface DRAM address register.
    pub struct AiDramAddrReg(pub u32): Debug {
        pub dram_addr: u32 [write_only] @ 0..24,
    }
}

bitfield! {
    /// Audio interface length register.
    pub struct AiLenReg(pub u32): Debug {
        pub transfer_length_v1: u32 @ 0..15,
        pub transfer_length_v2: u32 @ 0..18,
    }
}

bitfield! {
    /// Audio interface control register.
    pub struct AiControlReg(pub u32): Debug {
        pub dma_enable: bool [write_only] @ 0,
    }
}

bitfield! {
    /// Audio interface status register.
    pub struct AiStatusReg(pub u32): Debug {
        pub ai_busy: bool [read_only] @ 30,
        pub ai_full: bool [read_only] @ 31,
        pub clear_ai_intr: bool [write_only] @ 0,
    }
}

bitfield! {
    /// Audio interface DAC rate register.
    pub struct AiDacrateReg(pub u32): Debug {
        pub dacrate: u16 [write_only] @ 0..14,
    }
}

bitfield! {
    /// Audio interface bitrate register.
    pub struct AiBitrateReg(pub u32): Debug {
        pub bitrate: u8 [write_only] @ 0..4,
    }
}
