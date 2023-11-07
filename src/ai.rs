//! # Audio interface (AI)

use proc_bitfield::bitfield;

use crate::{fields, registers};

pub const AI_BASE_ADDR: u32 = 0x0450_0000;
pub const AI_OFFSET: u32 = 0xA000_0000;

registers! {
    /// # Audio interface (AI)
    AI_BASE_ADDR => Ai {
        /// Address of audio sample in DRAM.
        pub ai_dram_addr_reg: AiDramAddrReg,

        /// Length of audio sample.
        pub ai_len_reg: AiLenReg,

        /// Control.
        pub ai_control_reg: AiControlReg,

        /// Status.
        pub ai_status_reg: AiStatusReg,

        /// DAC rate.
        pub ai_dacrate_reg: AiDacrateReg,

        /// Bitrate.
        pub ai_bitrate_reg: AiBitrateReg,
    }
}

bitfield! {
    /// # `AI_DRAM_ADDR_REG`
    pub struct AiDramAddrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub starting_rdram_address: u32 [RdramAddress] @ 0..24,
    }
}

bitfield! {
    /// # `AI_LEN_REG`
    pub struct AiLenReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub transfer_length_v1: u32 [TransferLengthV1] @ 0..15,
        pub transfer_length_v2: u32 [TransferLengthV2] @ 0..18,
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
        pub dac_rate: u16 [write_only, set DacRate] @ 0..14,
    }
}

bitfield! {
    /// # `AI_BITRATE_REG`
    pub struct AiBitrateReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub bitrate: u8 [write_only, set Bitrate] @ 0..4,
    }
}

fields! [
    /// # Bitrate
    ux::u4 => Bitrate,

    /// # DAC rate
    ux::u14 => DacRate,

    /// # RDRAM address
    ux::u24 => RdramAddress,

    /// # Transfer length (v1.0)
    ux::u14 => TransferLengthV1,

    /// # Transfer length (v2.0)
    ux::u18 => TransferLengthV2,
];
