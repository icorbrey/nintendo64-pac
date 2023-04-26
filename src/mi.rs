//! # MIPS Interface Wrapper
//!
//! This module wraps the Nintendo 64's MIPS interface registers and provides
//! type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

use crate::{register_access, HARDWARE};

register_access!(0x0430_0000, Registers);

#[non_exhaustive]
pub struct MipsInterface {
    /// Contains getters and setters for `MI_INTR_MASK_REG`.
    pub interrupt_masks: InterruptMasks,

    /// Contains getters and setters for `MI_INTR_REG`.
    pub interrupts: Interrupts,

    /// Contains getters and setters for `MI_VERSION_REG`.
    pub version: Version,

    /// Contains getters and setters for `MI_MODE_REG`.
    pub mode: Mode,
}

impl MipsInterface {
    /// Returns ownership of the MIPS interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.mips_interface.drop(self) }
    }
}

/// A zero-size wrapper around `MI_MODE_REG`.
#[non_exhaustive]
pub struct Mode;

impl Mode {
    pub fn init_length(&self) -> u32 {
        registers().mode.read(MiModeReg::INIT_LENGTH)
    }

    pub fn init_mode(&self) -> bool {
        registers().mode.is_set(MiModeReg::INIT_MODE)
    }

    pub fn ebus_test_mode(&self) -> bool {
        registers().mode.is_set(MiModeReg::EBUS_TEST_MODE)
    }

    pub fn rdram_reg_mode(&self) -> bool {
        registers().mode.is_set(MiModeReg::RDRAM_REG_MODE)
    }

    pub fn set_init_length(&self, init_length: u32) {
        registers()
            .mode
            .write(MiModeReg::INIT_LENGTH.val(init_length))
    }

    pub fn clear_init_mode(&self) {
        registers().mode.write(MiModeReg::CLEAR_INIT_MODE::SET)
    }

    pub fn set_init_mode(&self) {
        registers().mode.write(MiModeReg::SET_INIT_MODE::SET)
    }

    pub fn clear_ebus_test_mode(&self) {
        registers().mode.write(MiModeReg::CLEAR_EBUS_TEST_MODE::SET)
    }

    pub fn set_ebus_test_mode(&self) {
        registers().mode.write(MiModeReg::SET_EBUS_TEST_MODE::SET)
    }

    pub fn clear_dp_interrupt(&self) {
        registers().mode.write(MiModeReg::CLEAR_DP_INTERRUPT::SET)
    }

    pub fn clear_rdram_reg_mode(&self) {
        registers().mode.write(MiModeReg::CLEAR_RDRAM_REG_MODE::SET)
    }

    pub fn set_rdram_reg_mode(&self) {
        registers().mode.write(MiModeReg::SET_RDRAM_REG_MODE::SET)
    }
}

/// A zero-size wrapper around `MI_VERSION_REG`.
#[non_exhaustive]
pub struct Version;

impl Version {
    pub fn io(&self) -> u32 {
        registers().version.read(MiVersionReg::IO)
    }

    pub fn rac(&self) -> u32 {
        registers().version.read(MiVersionReg::RAC)
    }

    pub fn rdp(&self) -> u32 {
        registers().version.read(MiVersionReg::RDP)
    }

    pub fn rsp(&self) -> u32 {
        registers().version.read(MiVersionReg::RSP)
    }
}

/// A zero-size wrapper around `MI_INTR_REG`.
#[non_exhaustive]
pub struct Interrupts;

impl Interrupts {
    pub fn sp_interrupt(&self) -> bool {
        registers().intr.is_set(MiIntrReg::SP_INTR)
    }

    pub fn si_interrupt(&self) -> bool {
        registers().intr.is_set(MiIntrReg::SI_INTR)
    }

    pub fn ai_interrupt(&self) -> bool {
        registers().intr.is_set(MiIntrReg::AI_INTR)
    }

    pub fn vi_interrupt(&self) -> bool {
        registers().intr.is_set(MiIntrReg::VI_INTR)
    }

    pub fn pi_interrupt(&self) -> bool {
        registers().intr.is_set(MiIntrReg::PI_INTR)
    }

    pub fn dp_interrupt(&self) -> bool {
        registers().intr.is_set(MiIntrReg::DP_INTR)
    }
}

/// A zero-size wrapper around `MI_INTR_MASK_REG`.
#[non_exhaustive]
pub struct InterruptMasks;

impl InterruptMasks {
    pub fn sp_interrupt_mask(&self) -> bool {
        registers().intr_mask.is_set(MiIntrMaskReg::SP_INTR_MASK)
    }

    pub fn si_interrupt_mask(&self) -> bool {
        registers().intr_mask.is_set(MiIntrMaskReg::SI_INTR_MASK)
    }

    pub fn ai_interrupt_mask(&self) -> bool {
        registers().intr_mask.is_set(MiIntrMaskReg::AI_INTR_MASK)
    }

    pub fn vi_interrupt_mask(&self) -> bool {
        registers().intr_mask.is_set(MiIntrMaskReg::VI_INTR_MASK)
    }

    pub fn pi_interrupt_mask(&self) -> bool {
        registers().intr_mask.is_set(MiIntrMaskReg::PI_INTR_MASK)
    }

    pub fn dp_interrupt_mask(&self) -> bool {
        registers().intr_mask.is_set(MiIntrMaskReg::DP_INTR_MASK)
    }

    pub fn clear_sp_interrupt_mask(&self) {
        registers()
            .intr_mask
            .write(MiIntrMaskReg::CLEAR_SP_MASK::SET)
    }

    pub fn set_sp_interrupt_mask(&self) {
        registers().intr_mask.write(MiIntrMaskReg::SET_SP_MASK::SET)
    }

    pub fn clear_si_interrupt_mask(&self) {
        registers()
            .intr_mask
            .write(MiIntrMaskReg::CLEAR_SI_MASK::SET)
    }

    pub fn set_si_interrupt_mask(&self) {
        registers().intr_mask.write(MiIntrMaskReg::SET_SI_MASK::SET)
    }

    pub fn clear_ai_interrupt_mask(&self) {
        registers()
            .intr_mask
            .write(MiIntrMaskReg::CLEAR_AI_MASK::SET)
    }

    pub fn set_ai_interrupt_mask(&self) {
        registers().intr_mask.write(MiIntrMaskReg::SET_AI_MASK::SET)
    }

    pub fn clear_vi_interrupt_mask(&self) {
        registers()
            .intr_mask
            .write(MiIntrMaskReg::CLEAR_VI_MASK::SET)
    }

    pub fn set_vi_interrupt_mask(&self) {
        registers().intr_mask.write(MiIntrMaskReg::SET_VI_MASK::SET)
    }

    pub fn clear_pi_interrupt_mask(&self) {
        registers()
            .intr_mask
            .write(MiIntrMaskReg::CLEAR_PI_MASK::SET)
    }

    pub fn set_pi_interrupt_mask(&self) {
        registers().intr_mask.write(MiIntrMaskReg::SET_PI_MASK::SET)
    }

    pub fn clear_dp_interrupt_mask(&self) {
        registers()
            .intr_mask
            .write(MiIntrMaskReg::CLEAR_DP_MASK::SET)
    }

    pub fn set_dp_interrupt_mask(&self) {
        registers().intr_mask.write(MiIntrMaskReg::SET_DP_MASK::SET)
    }
}

register_structs! {
    Registers {
        (0x0000 => pub mode: ReadWrite<u32, MiModeReg::Register>),
        (0x0004 => pub version: ReadOnly<u32, MiVersionReg::Register>),
        (0x0008 => pub intr: ReadOnly<u32, MiIntrReg::Register>),
        (0x000C => pub intr_mask: ReadWrite<u32, MiIntrMaskReg::Register>),
        (0x0010 => @END),
    }
}

register_bitfields! {
    u32,

    MiModeReg [
        INIT_LENGTH          OFFSET(0)  NUMBITS(7) [],
        INIT_MODE            OFFSET(7)  NUMBITS(1) [],
        EBUS_TEST_MODE       OFFSET(8)  NUMBITS(1) [],
        RDRAM_REG_MODE       OFFSET(9)  NUMBITS(1) [],
        CLEAR_INIT_MODE      OFFSET(7)  NUMBITS(1) [],
        SET_INIT_MODE        OFFSET(8)  NUMBITS(1) [],
        CLEAR_EBUS_TEST_MODE OFFSET(9)  NUMBITS(1) [],
        SET_EBUS_TEST_MODE   OFFSET(10) NUMBITS(1) [],
        CLEAR_DP_INTERRUPT   OFFSET(11) NUMBITS(1) [],
        CLEAR_RDRAM_REG_MODE OFFSET(12) NUMBITS(1) [],
        SET_RDRAM_REG_MODE   OFFSET(13) NUMBITS(1) [],
    ],

    MiVersionReg [
        IO                   OFFSET(0)  NUMBITS(8) [],
        RAC                  OFFSET(8)  NUMBITS(8) [],
        RDP                  OFFSET(16) NUMBITS(8) [],
        RSP                  OFFSET(24) NUMBITS(8) [],
    ],

    MiIntrReg [
        SP_INTR              OFFSET(0)  NUMBITS(1) [],
        SI_INTR              OFFSET(1)  NUMBITS(1) [],
        AI_INTR              OFFSET(2)  NUMBITS(1) [],
        VI_INTR              OFFSET(3)  NUMBITS(1) [],
        PI_INTR              OFFSET(4)  NUMBITS(1) [],
        DP_INTR              OFFSET(5)  NUMBITS(1) [],
    ],

    MiIntrMaskReg [
        SP_INTR_MASK         OFFSET(0)  NUMBITS(1) [],
        SI_INTR_MASK         OFFSET(1)  NUMBITS(1) [],
        AI_INTR_MASK         OFFSET(2)  NUMBITS(1) [],
        VI_INTR_MASK         OFFSET(3)  NUMBITS(1) [],
        PI_INTR_MASK         OFFSET(4)  NUMBITS(1) [],
        DP_INTR_MASK         OFFSET(5)  NUMBITS(1) [],

        CLEAR_SP_MASK        OFFSET(0)  NUMBITS(1) [],
        SET_SP_MASK          OFFSET(1)  NUMBITS(1) [],
        CLEAR_SI_MASK        OFFSET(2)  NUMBITS(1) [],
        SET_SI_MASK          OFFSET(3)  NUMBITS(1) [],
        CLEAR_AI_MASK        OFFSET(4)  NUMBITS(1) [],
        SET_AI_MASK          OFFSET(5)  NUMBITS(1) [],
        CLEAR_VI_MASK        OFFSET(6)  NUMBITS(1) [],
        SET_VI_MASK          OFFSET(7)  NUMBITS(1) [],
        CLEAR_PI_MASK        OFFSET(8)  NUMBITS(1) [],
        SET_PI_MASK          OFFSET(9)  NUMBITS(1) [],
        CLEAR_DP_MASK        OFFSET(10) NUMBITS(1) [],
        SET_DP_MASK          OFFSET(11) NUMBITS(1) [],
    ]
}
