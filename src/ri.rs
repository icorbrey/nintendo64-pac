//! # RDRAM Interface Wrapper
//!
//! This module wraps the Nintendo 64's RDRAM interface registers and provides
//! type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite, WriteOnly},
};

use crate::{register_access, HARDWARE};

register_access!(0x0470_0000, Registers);

/// A zero-size wrapper around the Nintendo 64's RDRAM interface registers.
#[non_exhaustive]
pub struct RdramInterface {
    /// Contains getters and setters for `RI_CURRENT_LOAD_REG`.
    pub current_load: CurrentLoad,

    /// Contains getters and setters for `RI_WERROR_REG`.
    pub write_error: WriteError,

    /// Contains getters and setters for `RI_RERROR_REG`.
    pub read_error: ReadError,

    /// Contains getters and setters for `RI_LATENCY_REG`.
    pub latency: Latency,

    /// Contains getters and setters for `RI_REFRESH_REG`.
    pub refresh: Refresh,

    /// Contains getters and setters for `RI_CONFIG_REG`.
    pub config: Config,

    /// Contains getters and setters for `RI_SELECT_REG`.
    pub select: Select,

    /// Contains getters and setters for `RI_MODE_REG`.
    pub mode: Mode,
}

impl RdramInterface {
    /// Returns ownership of the RDRAM interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.rdram_interface.drop(self) }
    }
}

/// A zero-size wrapper around `RI_MODE_REG`.
#[non_exhaustive]
pub struct Mode;

impl Mode {
    pub fn operating_mode(&self) -> u32 {
        registers().mode.read(RiModeReg::OPERATING_MODE)
    }

    pub fn stop_transmit_active(&self) -> bool {
        registers().mode.is_set(RiModeReg::STOP_T_ACTIVE)
    }

    pub fn stop_receive_active(&self) -> bool {
        registers().mode.is_set(RiModeReg::STOP_R_ACTIVE)
    }

    pub fn set_operating_mode(&self, operating_mode: u32) {
        registers()
            .mode
            .write(RiModeReg::OPERATING_MODE.val(operating_mode))
    }

    pub fn set_stop_transmit_active(&self, stop_transmit_active: bool) {
        registers()
            .mode
            .write(RiModeReg::STOP_T_ACTIVE.val(stop_transmit_active.into()))
    }

    pub fn set_stop_receive_active(&self, stop_receive_active: bool) {
        registers()
            .mode
            .write(RiModeReg::STOP_R_ACTIVE.val(stop_receive_active.into()))
    }
}

/// A zero-size wrapper around `RI_CONFIG_REG`.
#[non_exhaustive]
pub struct Config;

impl Config {
    pub fn current_control_input(&self) -> u32 {
        registers().config.read(RiConfigReg::CURRENT_CONTROL_INPUT)
    }

    pub fn is_current_control_enabled(&self) -> bool {
        registers()
            .config
            .is_set(RiConfigReg::CURRENT_CONTROL_ENABLE)
    }

    pub fn set_current_control_input(&self, current_control_input: u32) {
        registers()
            .config
            .write(RiConfigReg::CURRENT_CONTROL_INPUT.val(current_control_input))
    }

    pub fn enable_current_control(&self) {
        registers()
            .config
            .write(RiConfigReg::CURRENT_CONTROL_ENABLE::SET);
    }

    pub fn disable_current_control(&self) {
        registers()
            .config
            .write(RiConfigReg::CURRENT_CONTROL_ENABLE::CLEAR);
    }

    pub fn set_is_current_control_enabled(&self, is_current_control_enabled: bool) {
        registers()
            .config
            .write(RiConfigReg::CURRENT_CONTROL_ENABLE.val(is_current_control_enabled.into()))
    }
}

/// A zero-size wrapper around `RI_CURRENT_LOAD_REG`.
#[non_exhaustive]
pub struct CurrentLoad;

impl CurrentLoad {
    pub fn update_current_control(&self) {
        registers()
            .current_load
            .write(RiCurrentLoadReg::UPDATE_CURRENT_CONTROL::SET)
    }
}

/// A zero-size wrapper around `RI_SELECT_REG`.
#[non_exhaustive]
pub struct Select;

impl Select {
    pub fn receive_select(&self) -> u32 {
        registers().select.read(RiSelectReg::RECEIVE_SELECT)
    }

    pub fn transmit_select(&self) -> u32 {
        registers().select.read(RiSelectReg::TRANSMIT_SELECT)
    }

    pub fn set_receive_select(&self, receive_select: u32) {
        registers()
            .select
            .write(RiSelectReg::RECEIVE_SELECT.val(receive_select))
    }

    pub fn set_transmit_select(&self, transmit_select: u32) {
        registers()
            .select
            .write(RiSelectReg::TRANSMIT_SELECT.val(transmit_select))
    }
}

/// A zero-size wrapper around `RI_REFRESH_REG`.
#[non_exhaustive]
pub struct Refresh;

impl Refresh {
    pub fn clean_delay(&self) -> u32 {
        registers().refresh.read(RiRefreshReg::CLEAN_REFRESH_DELAY)
    }

    pub fn dirty_delay(&self) -> u32 {
        registers().refresh.read(RiRefreshReg::DIRTY_REFRESH_DELAY)
    }

    pub fn bank(&self) -> bool {
        registers().refresh.is_set(RiRefreshReg::REFRESH_BANK)
    }

    pub fn is_enabled(&self) -> bool {
        registers().refresh.is_set(RiRefreshReg::REFRESH_ENABLE)
    }

    pub fn optimize(&self) -> bool {
        registers().refresh.is_set(RiRefreshReg::REFRESH_OPTIMIZE)
    }

    pub fn set_clean_delay(&self, clean_delay: u32) {
        registers()
            .refresh
            .write(RiRefreshReg::CLEAN_REFRESH_DELAY.val(clean_delay))
    }

    pub fn set_dirty_delay(&self, dirty_delay: u32) {
        registers()
            .refresh
            .write(RiRefreshReg::DIRTY_REFRESH_DELAY.val(dirty_delay))
    }

    pub fn set_bank(&self, bank: bool) {
        registers()
            .refresh
            .write(RiRefreshReg::REFRESH_BANK.val(bank.into()))
    }

    pub fn set_is_enabled(&self, is_enabled: bool) {
        registers()
            .refresh
            .write(RiRefreshReg::REFRESH_ENABLE.val(is_enabled.into()))
    }

    pub fn enable(&self) {
        registers().refresh.write(RiRefreshReg::REFRESH_ENABLE::SET)
    }

    pub fn disable(&self) {
        registers()
            .refresh
            .write(RiRefreshReg::REFRESH_ENABLE::CLEAR)
    }

    pub fn set_optimize(&self, optimize: bool) {
        registers()
            .refresh
            .write(RiRefreshReg::REFRESH_OPTIMIZE.val(optimize.into()))
    }
}

/// A zero-size wrapper around `RI_LATENCY_REG`.
#[non_exhaustive]
pub struct Latency;

impl Latency {
    pub fn get(&self) -> u32 {
        registers().latency.read(RiLatencyReg::DMA_LATENCY)
    }

    pub fn set(&self, latency: u32) {
        registers()
            .latency
            .write(RiLatencyReg::DMA_LATENCY.val(latency))
    }
}

/// A zero-size wrapper around `RI_RERROR_REG`.
#[non_exhaustive]
pub struct ReadError;

impl ReadError {
    pub fn nack_error(&self) -> bool {
        registers().read_error.is_set(RiRErrorReg::NACK_ERROR)
    }

    pub fn ack_error(&self) -> bool {
        registers().read_error.is_set(RiRErrorReg::ACK_ERROR)
    }
}

/// A zero-size wrapper around `RI_WERROR_REG`.
#[non_exhaustive]
pub struct WriteError;

impl WriteError {
    pub fn clear_errors(&self) {
        registers()
            .write_error
            .write(RiWErrorReg::CLEAR_ERRORS::SET)
    }
}

register_structs! {
    Registers {
        (0x0000 => pub mode: ReadWrite<u32, RiModeReg::Register>),
        (0x0004 => pub config: ReadWrite<u32, RiConfigReg::Register>),
        (0x0008 => pub current_load: WriteOnly<u32, RiCurrentLoadReg::Register>),
        (0x000C => pub select: ReadWrite<u32, RiSelectReg::Register>),
        (0x0010 => pub refresh: ReadWrite<u32, RiRefreshReg::Register>),
        (0x0014 => pub latency: ReadWrite<u32, RiLatencyReg::Register>),
        (0x0018 => pub read_error: ReadOnly<u32, RiRErrorReg::Register>),
        (0x001C => pub write_error: WriteOnly<u32, RiWErrorReg::Register>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    RiModeReg [
        OPERATING_MODE         OFFSET(0)  NUMBITS(2) [],
        STOP_T_ACTIVE   OFFSET(2)  NUMBITS(1) [],
        STOP_R_ACTIVE    OFFSET(3)  NUMBITS(1) [],
    ],

    RiConfigReg [
        CURRENT_CONTROL_INPUT  OFFSET(0)  NUMBITS(6) [],
        CURRENT_CONTROL_ENABLE OFFSET(6)  NUMBITS(1) [],
    ],

    RiCurrentLoadReg [
        UPDATE_CURRENT_CONTROL OFFSET(0)  NUMBITS(1) [],
    ],

    RiSelectReg [
        RECEIVE_SELECT         OFFSET(0)  NUMBITS(2) [],
        TRANSMIT_SELECT        OFFSET(2)  NUMBITS(2) [],
    ],

    RiRefreshReg [
        CLEAN_REFRESH_DELAY    OFFSET(0)  NUMBITS(8) [],
        DIRTY_REFRESH_DELAY    OFFSET(8)  NUMBITS(8) [],
        REFRESH_BANK           OFFSET(16) NUMBITS(1) [],
        REFRESH_ENABLE         OFFSET(17) NUMBITS(1) [],
        REFRESH_OPTIMIZE       OFFSET(18) NUMBITS(1) [],
    ],

    RiLatencyReg [
        DMA_LATENCY            OFFSET(0)  NUMBITS(4) [],
    ],

    RiRErrorReg [
        NACK_ERROR             OFFSET(0)  NUMBITS(1) [],
        ACK_ERROR              OFFSET(1)  NUMBITS(1) [],
    ],

    RiWErrorReg [
        CLEAR_ERRORS           OFFSET(0)  NUMBITS(1) [],
    ],
}
