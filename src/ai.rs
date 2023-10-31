//! # Audio Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # Audio Interface
pub struct Ai;

impl_interface!(Ai, AiRegisters, 0x0450_0000);

/// # Audio Interface Register Block
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

bitfield! {
    /// # Audio Interface DRAM Address Register
    pub struct AiDramAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_rdram_address: u32 [write_only, try_set RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # Audio Interface Length Register
    pub struct AiLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub transfer_length_v1: u32 [get TransferLengthV1, try_set TransferLengthV1] @ 0..15,
        pub transfer_length_v2: u32 [get TransferLengthV2, try_set TransferLengthV2] @ 0..18,
    }
}
bitfield! {
    /// # Audio Interface Control Register
    pub struct AiControlReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub is_dma_enabled: bool [write_only] @ 0,
    }
}

bitfield! {
    /// # Audio Interface Status Register
    pub struct AiStatusReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub ai_busy: bool [read_only] @ 30,
        pub ai_full: bool [read_only] @ 31,
        pub clear_ai_intr: bool [write_only] @ 0,
    }
}

bitfield! {
    /// # Audio Interface DAC Rate Register
    pub struct AiDacrateReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub dac_rate: u16 [write_only, try_set DacRate] @ 0..14,
    }
}

bitfield! {
    /// # Audio Interface Bitrate Register
    pub struct AiBitrateReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub bitrate: u8 [write_only, try_set Bitrate] @ 0..4,
    }
}

/// # RDRAM Address
///
/// This field is 8-bit aligned.
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
///
/// The DAC sample rate is equal to `vid_clock / (dperiod + 1)`. `dperiod + 1`
/// must be greater than or equal to `66 * (aclockhp + 1)`.
pub struct DacRate(pub u16);

impl_deref!(DacRate, u16);
impl_set!(DacRate, u16, 0..14);

/// # Bitrate
///
/// The bitrate is equal to `abus_clock_half_period_register - aclockhp`. The
/// DAC clock rate is `vid_clock / (2 * (aclockhp + 1))`. The ABUS clock stops
/// if `aclockhp` is zero.
pub struct Bitrate(pub u8);

impl_deref!(Bitrate, u8);
impl_set!(Bitrate, u8, 0..4);
