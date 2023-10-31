//! # Control

use proc_bitfield::bitfield;

bitfield! {
    /// Audio interface control register.
    pub struct AiControlReg(pub u32): Debug {
        /// Raw register access.
        pub raw: u32 @ ..,

        pub dma_enable: bool [write_only] @ 0,
    }
}
