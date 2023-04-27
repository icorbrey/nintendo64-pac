//! # Peripheral Interface Wrapper
//!
//! This module wraps the Nintendo 64's peripheral interface registers and
//! provides type- and memory safe ways of interacting with it.

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};

use crate::{register_access, HARDWARE};

register_access!(0x0460_0000, Registers);

#[non_exhaustive]
pub struct PeripheralInterface {
    pub cart_address: CartAddress,
    pub dram_address: DramAddress,
    pub write_length: WriteLength,
    pub read_length: ReadLength,
    pub domain_1: Domain1,
    pub domain_2: Domain2,
    pub status: Status,
}

impl PeripheralInterface {
    /// Returns ownership of the peripheral interface registers to
    /// [`HARDWARE`][crate::HARDWARE].
    pub fn drop(self) {
        unsafe { HARDWARE.peripheral_interface.drop(self) };
    }
}

#[non_exhaustive]
pub struct DramAddress;

impl DramAddress {
    pub fn get(&self) -> u32 {
        registers()
            .dram_addr
            .read(PiDramAddrReg::STARTING_RDRAM_ADDRESS)
    }

    pub fn set(&self, dram_address: u32) {
        registers()
            .dram_addr
            .write(PiDramAddrReg::STARTING_RDRAM_ADDRESS.val(dram_address))
    }
}

#[non_exhaustive]
pub struct CartAddress;

impl CartAddress {
    pub fn get(&self) -> u32 {
        registers()
            .cart_addr
            .read(PiCartAddrReg::STARTING_AD16_ADDRESS)
    }

    pub fn set(&self, cart_address: u32) {
        registers()
            .cart_addr
            .write(PiCartAddrReg::STARTING_AD16_ADDRESS.val(cart_address))
    }
}

#[non_exhaustive]
pub struct ReadLength;

impl ReadLength {
    pub fn get(&self) -> u32 {
        registers().rd_len.read(PiRdLenReg::READ_DATA_LENGTH)
    }

    pub fn set(&self, read_length: u32) {
        registers()
            .rd_len
            .write(PiRdLenReg::READ_DATA_LENGTH.val(read_length))
    }
}

#[non_exhaustive]
pub struct WriteLength;

impl WriteLength {
    pub fn get(&self) -> u32 {
        registers().wr_len.read(PiWrLenReg::WRITE_DATA_LENGTH)
    }

    pub fn set(&self, write_length: u32) {
        registers()
            .wr_len
            .write(PiWrLenReg::WRITE_DATA_LENGTH.val(write_length))
    }
}

#[non_exhaustive]
pub struct Status;

impl Status {
    pub fn dma_busy(&self) -> bool {
        registers().status.is_set(PiStatusReg::DMA_BUSY)
    }

    pub fn io_busy(&self) -> bool {
        registers().status.is_set(PiStatusReg::IO_BUSY)
    }

    pub fn error(&self) -> bool {
        registers().status.is_set(PiStatusReg::ERROR)
    }

    pub fn reset_controller(&self) {
        registers().status.write(PiStatusReg::RESET_CONTROLLER::SET)
    }

    pub fn clear_interrupt(&self) {
        registers().status.write(PiStatusReg::CLEAR_INTR::SET)
    }
}

#[non_exhaustive]
pub struct Domain1 {
    pub release_duration: Domain1ReleaseDuration,
    pub pulse_width: Domain1PulseWidth,
    pub page_size: Domain1PageSize,
    pub latency: Domain1Latency,
}

#[non_exhaustive]
pub struct Domain1Latency;

impl Domain1Latency {
    pub fn get(&self) -> u32 {
        registers().bsd_dom1_lat.read(PiBsdDomXLatReg::LATENCY)
    }

    pub fn set(&self, latency: u32) {
        registers()
            .bsd_dom1_lat
            .write(PiBsdDomXLatReg::LATENCY.val(latency))
    }
}

#[non_exhaustive]
pub struct Domain1PulseWidth;

impl Domain1PulseWidth {
    pub fn get(&self) -> u32 {
        registers().bsd_dom1_pwd.read(PiBsdDomXPwdReg::PULSE_WIDTH)
    }

    pub fn set(&self, pulse_width: u32) {
        registers()
            .bsd_dom1_pwd
            .write(PiBsdDomXPwdReg::PULSE_WIDTH.val(pulse_width))
    }
}

#[non_exhaustive]
pub struct Domain1PageSize;

impl Domain1PageSize {
    pub fn get(&self) -> u32 {
        registers().bsd_dom1_pgs.read(PiBsdDomXPgsReg::PAGE_SIZE)
    }

    pub fn set(&self, page_size: u32) {
        registers()
            .bsd_dom1_pgs
            .write(PiBsdDomXPgsReg::PAGE_SIZE.val(page_size))
    }
}

#[non_exhaustive]
pub struct Domain1ReleaseDuration;

impl Domain1ReleaseDuration {
    pub fn get(&self) -> u32 {
        registers()
            .bsd_dom1_rls
            .read(PiBsdDomXRlsReg::RELEASE_DURATION)
    }

    pub fn set(&self, release_duration: u32) {
        registers()
            .bsd_dom1_rls
            .write(PiBsdDomXRlsReg::RELEASE_DURATION.val(release_duration))
    }
}

#[non_exhaustive]
pub struct Domain2 {
    pub release_duration: Domain2ReleaseDuration,
    pub pulse_width: Domain2PulseWidth,
    pub page_size: Domain2PageSize,
    pub latency: Domain2Latency,
}

#[non_exhaustive]
pub struct Domain2Latency;

impl Domain2Latency {
    pub fn get(&self) -> u32 {
        registers().bsd_dom2_lat.read(PiBsdDomXLatReg::LATENCY)
    }

    pub fn set(&self, latency: u32) {
        registers()
            .bsd_dom2_lat
            .write(PiBsdDomXLatReg::LATENCY.val(latency))
    }
}

#[non_exhaustive]
pub struct Domain2PulseWidth;

impl Domain2PulseWidth {
    pub fn get(&self) -> u32 {
        registers().bsd_dom2_pwd.read(PiBsdDomXPwdReg::PULSE_WIDTH)
    }

    pub fn set(&self, pulse_width: u32) {
        registers()
            .bsd_dom2_pwd
            .write(PiBsdDomXPwdReg::PULSE_WIDTH.val(pulse_width))
    }
}

#[non_exhaustive]
pub struct Domain2PageSize;

impl Domain2PageSize {
    pub fn get(&self) -> u32 {
        registers().bsd_dom2_pgs.read(PiBsdDomXPgsReg::PAGE_SIZE)
    }

    pub fn set(&self, page_size: u32) {
        registers()
            .bsd_dom2_pgs
            .write(PiBsdDomXPgsReg::PAGE_SIZE.val(page_size))
    }
}

#[non_exhaustive]
pub struct Domain2ReleaseDuration;

impl Domain2ReleaseDuration {
    pub fn get(&self) -> u32 {
        registers()
            .bsd_dom2_rls
            .read(PiBsdDomXRlsReg::RELEASE_DURATION)
    }

    pub fn set(&self, release_duration: u32) {
        registers()
            .bsd_dom2_rls
            .write(PiBsdDomXRlsReg::RELEASE_DURATION.val(release_duration))
    }
}

register_structs! {
    Registers {
        (0x0000 => pub dram_addr: ReadWrite<u32, PiDramAddrReg::Register>),
        (0x0004 => pub cart_addr: ReadWrite<u32, PiCartAddrReg::Register>),
        (0x0008 => pub rd_len: ReadWrite<u32, PiRdLenReg::Register>),
        (0x000C => pub wr_len: ReadWrite<u32, PiWrLenReg::Register>),
        (0x0010 => pub status: ReadWrite<u32, PiStatusReg::Register>),
        (0x0014 => pub bsd_dom1_lat: ReadWrite<u32, PiBsdDomXLatReg::Register>),
        (0x0018 => pub bsd_dom1_pwd: ReadWrite<u32, PiBsdDomXPwdReg::Register>),
        (0x001C => pub bsd_dom1_pgs: ReadWrite<u32, PiBsdDomXPgsReg::Register>),
        (0x0020 => pub bsd_dom1_rls: ReadWrite<u32, PiBsdDomXRlsReg::Register>),
        (0x0024 => pub bsd_dom2_lat: ReadWrite<u32, PiBsdDomXLatReg::Register>),
        (0x0028 => pub bsd_dom2_pwd: ReadWrite<u32, PiBsdDomXPwdReg::Register>),
        (0x002C => pub bsd_dom2_pgs: ReadWrite<u32, PiBsdDomXPgsReg::Register>),
        (0x0030 => pub bsd_dom2_rls: ReadWrite<u32, PiBsdDomXRlsReg::Register>),
        (0x0034 => @END),
    }
}

register_bitfields! {
    u32,

    PiDramAddrReg [
        STARTING_RDRAM_ADDRESS OFFSET(0) NUMBITS(24) [],
    ],

    PiCartAddrReg [
        STARTING_AD16_ADDRESS  OFFSET(0) NUMBITS(32) [],
    ],

    PiRdLenReg [
        READ_DATA_LENGTH       OFFSET(0) NUMBITS(24) [],
    ],

    PiWrLenReg [
        WRITE_DATA_LENGTH      OFFSET(0) NUMBITS(24) [],
    ],

    PiStatusReg [
        DMA_BUSY               OFFSET(0) NUMBITS(1)  [],
        IO_BUSY                OFFSET(1) NUMBITS(1)  [],
        ERROR                  OFFSET(2) NUMBITS(1)  [],

        RESET_CONTROLLER       OFFSET(0) NUMBITS(1)  [],
        CLEAR_INTR             OFFSET(1) NUMBITS(1)  [],
    ],

    PiBsdDomXLatReg [
        LATENCY                OFFSET(0) NUMBITS(8)  [],
    ],

    PiBsdDomXPwdReg [
        PULSE_WIDTH            OFFSET(0) NUMBITS(8)  [],
    ],

    PiBsdDomXPgsReg [
        PAGE_SIZE              OFFSET(0) NUMBITS(4)  [],
    ],

    PiBsdDomXRlsReg [
        RELEASE_DURATION       OFFSET(0) NUMBITS(2)  [],
    ],
}
