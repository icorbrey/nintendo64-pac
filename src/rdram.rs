//! # RDRAM Wrapper

use core::ops::Deref;

const RDRAM_BASE_REG: u32 = 0x03F0_0000;

/// RDRAM.
pub struct Rdram;

impl Rdram {
    pub fn ptr() -> *const RdramRegisters {
        RDRAM_BASE_REG as *const _
    }
}

impl Deref for Rdram {
    type Target = RdramRegisters;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}

/// RDRAM registers.
#[repr(C)]
pub struct RdramRegisters;
