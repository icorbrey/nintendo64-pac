//! # Audio Interface

pub mod bitrate;
pub mod control;
pub mod dacrate;
pub mod dram_addr;
pub mod len;
pub mod status;

use core::ops::Deref;

pub use self::{
    bitrate::AiBitrateReg, control::AiControlReg, dacrate::AiDacrateReg, dram_addr::AiDramAddrReg,
    len::AiLenReg, status::AiStatusReg,
};

const AI_BASE_REG: u32 = 0x0450_0000;

/// Audio interface.
pub struct Ai;

impl Ai {
    pub fn ptr() -> *const AiRegisters {
        AI_BASE_REG as *const _
    }
}

unsafe impl Sync for Ai {}

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

    /// `0x08` - Control
    pub ai_control_reg: AiControlReg,

    /// `0x0C` - Status
    pub ai_status_reg: AiStatusReg,

    /// `0x10` - DAC rate
    pub ai_dacrate_reg: AiDacrateReg,

    /// `0x14` - Bitrate
    pub ai_bitrate_reg: AiBitrateReg,
}
