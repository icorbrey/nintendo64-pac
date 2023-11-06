//! # Audio Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

pub const AI_BASE_ADDR: u32 = 0x0450_0000;
pub const AI_OFFSET: u32 = 0xA000_0000;

/// # Audio interface (AI)
pub struct Ai;

impl_interface!(Ai, AiRegisters, AI_BASE_ADDR);

/// # AI register block
#[repr(C)]
pub struct AiRegisters {
    /// Address of audio sample in DRAM.
    pub ai_dram_addr_reg: AiDramAddrReg,

    /// Length of audio sample.
    pub ai_len_reg: AiLenReg,

    /// Control
    pub ai_control_reg: AiControlReg,

    /// Status
    pub ai_status_reg: AiStatusReg,

    /// DAC rate
    pub ai_dacrate_reg: AiDacrateReg,

    /// Bitrate
    pub ai_bitrate_reg: AiBitrateReg,
}

bitfield! {
    /// # `AI_DRAM_ADDR_REG`
    pub struct AiDramAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_rdram_address: u32 [write_only, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # `AI_LEN_REG`
    pub struct AiLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub transfer_length_v1: u32 [get TransferLengthV1, try_set TransferLengthV1] @ 0..15,
        pub transfer_length_v2: u32 [get TransferLengthV2, try_set TransferLengthV2] @ 0..18,
    }
}
bitfield! {
    /// # `AI_CONTROL_REG`
    pub struct AiControlReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dma_enable: bool [write_only] @ 0,
    }
}

bitfield! {
    /// # `AI_STATUS_REG`
    pub struct AiStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub ai_busy: bool [read_only] @ 30,
        pub ai_full: bool [read_only] @ 31,
        pub clear_ai_intr: bool [write_only] @ 0,
    }
}

bitfield! {
    /// # `AI_DACRATE_REG`
    pub struct AiDacrateReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dac_rate: u16 [write_only, try_set DacRate] @ 0..14,
    }
}

bitfield! {
    /// # `AI_BITRATE_REG`
    pub struct AiBitrateReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub bitrate: u8 [write_only, try_set Bitrate] @ 0..4,
    }
}

/// # RDRAM Address
pub struct RdramAddress(pub u32);

impl_deref!(RdramAddress, u32);
impl_set!(RdramAddress, u32, 0..24);

/// # Transfer Length (v1.0)
#[derive(Debug)]
pub struct TransferLengthV1(pub u32);

impl_deref!(TransferLengthV1, u32);
impl_get!(TransferLengthV1, u32);
impl_set!(TransferLengthV1, u32, 0..15);

/// # Transfer Length (v2.0)
#[derive(Debug)]
pub struct TransferLengthV2(pub u32);

impl_deref!(TransferLengthV2, u32);
impl_get!(TransferLengthV2, u32);
impl_set!(TransferLengthV2, u32, 0..18);

/// # DAC Rate
pub struct DacRate(pub u16);

impl_deref!(DacRate, u16);
impl_set!(DacRate, u16, 0..14);

/// # Bitrate
pub struct Bitrate(pub u8);

impl_deref!(Bitrate, u8);
impl_set!(Bitrate, u8, 0..4);
