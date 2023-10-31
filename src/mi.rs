//! # MIPS Interface

use core::ops::Deref;

use proc_bitfield::bitfield;

use crate::{impl_deref, impl_get, impl_interface, impl_set};

/// # MIPS Interface
pub struct Mi;

impl_interface!(Mi, MiRegisters, 0x0430_0000);

/// # MIPS Interface Register Block
#[repr(C)]
pub struct MiRegisters {
    /// `0x00` - Init Mode
    pub mi_init_mode_reg: MiInitModeReg,

    /// `0x04` - Version
    pub mi_version_reg: MiVersionReg,

    /// `0x08` - Interrupts
    pub mi_intr_reg: MiIntrReg,

    /// `0x0C` - Interrupt Masks
    pub mi_intr_mask_reg: MiIntrMaskReg,
}

bitfield! {
    /// # MIPS Interface Init Mode Register
    pub struct MiInitModeReg(pub u32): Debug {
        pub raw: u32 @ ..,

        pub init_length: u8 [get InitLength, try_set InitLength] @ 0..7,

        pub init_mode: bool [read_only] @ 7,
        pub ebus_test_mode: bool [read_only] @ 8,
        pub rdram_reg_mode: bool [read_only] @ 9,

        pub clear_init_mode: bool [write_only] @ 7,
        pub set_init_mode: bool [write_only] @ 8,
        pub clear_ebus_test_mode: bool [write_only] @ 9,
        pub set_ebus_test_mode: bool [write_only] @ 10,
        pub clear_dp_intr: bool [write_only] @ 11,
        pub clear_rdram_reg: bool [write_only] @ 12,
        pub set_dram_reg_mode: bool [write_only] @ 13,
    }
}

bitfield! {
    /// # MIPS Interface Version Register
    pub struct MiVersionReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub io: u8 [read_only, get Version] @ 0..8,
        pub rac: u8 [read_only, get Version] @ 8..16,
        pub rdp: u8 [read_only, get Version] @ 16..24,
        pub rsp: u8 [read_only, get Version] @ 24..32,
    }
}

bitfield! {
    /// # MIPS Interface Interrupt Register
    pub struct MiIntrReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub sp_intr: bool [read_only] @ 0,
        pub si_intr: bool [read_only] @ 1,
        pub ai_intr: bool [read_only] @ 2,
        pub vi_intr: bool [read_only] @ 3,
        pub pi_intr: bool [read_only] @ 4,
        pub dp_intr: bool [read_only] @ 5,
    }
}

bitfield! {
    /// # MIPS Interface Interrupt Mask Register
    pub struct MiIntrMaskReg(pub u32): Debug {
        pub raw: u32 @ ..,
        pub sp_intr_mask: bool [read_only] @ 0,
        pub si_intr_mask: bool [read_only] @ 1,
        pub ai_intr_mask: bool [read_only] @ 2,
        pub vi_intr_mask: bool [read_only] @ 3,
        pub pi_intr_mask: bool [read_only] @ 4,
        pub dp_intr_mask: bool [read_only] @ 5,
        pub clear_sp_mask: bool [write_only] @ 0,
        pub set_sp_mask: bool [write_only] @ 1,
        pub clear_si_mask: bool [write_only] @ 2,
        pub set_si_mask: bool [write_only] @ 3,
        pub clear_ai_mask: bool [write_only] @ 4,
        pub set_ai_mask: bool [write_only] @ 5,
        pub clear_vi_mask: bool [write_only] @ 6,
        pub set_vi_mask: bool [write_only] @ 7,
        pub clear_pi_mask: bool [write_only] @ 8,
        pub set_pi_mask: bool [write_only] @ 9,
        pub clear_dp_mask: bool [write_only] @ 10,
        pub set_dp_mask: bool [write_only] @ 11,
    }
}

/// # Init Length
#[derive(Debug)]
pub struct InitLength(pub u8);

impl_deref!(InitLength, u8);
impl_get!(InitLength, u8);
impl_set!(InitLength, u8, 0..7);

/// # Version
#[derive(Debug)]
pub struct Version(pub u8);

impl_deref!(Version, u8);
impl_get!(Version, u8);
