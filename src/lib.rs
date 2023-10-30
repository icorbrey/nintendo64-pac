//! # Nintendo 64 PAC

#![no_std]

use core::mem::replace;

pub mod ai;
pub mod dpc;
pub mod dps;
pub mod mi;
pub mod pc;
pub mod pi;
pub mod rdram;
pub mod ri;
pub mod si;
pub mod sp;
pub mod vi;

pub mod prelude {
    pub use super::ai::Ai;
    pub use super::dpc::Dpc;
    pub use super::rdram::Rdram;
    pub use super::sp::Sp;
}

/// A global, static reference to the hardware peripherals of the Nintendo 64.
pub static mut HARDWARE: Hardware = Hardware {
    peripheral_interface: Peripheral::new(pi::PeripheralInterface {
        cart_address: pi::CartAddress,
        dram_address: pi::DramAddress,
        write_length: pi::WriteLength,
        read_length: pi::ReadLength,
        domain_1: pi::Domain1 {
            release_duration: pi::Domain1ReleaseDuration,
            pulse_width: pi::Domain1PulseWidth,
            page_size: pi::Domain1PageSize,
            latency: pi::Domain1Latency,
        },
        domain_2: pi::Domain2 {
            release_duration: pi::Domain2ReleaseDuration,
            pulse_width: pi::Domain2PulseWidth,
            page_size: pi::Domain2PageSize,
            latency: pi::Domain2Latency,
        },
        status: pi::Status,
    }),
    serial_interface: Peripheral::new(si::SerialInterface {
        pif_address_write_64_bits: si::PifAddressWrite64Bits,
        pif_address_read_64_bits: si::PifAddressRead64Bits,
        dram_address: si::DramAddress,
        status: si::Status,
    }),
    program_counter: Peripheral::new(pc::ProgramCounter {
        imem_bist: pc::ImemBist,
        control: pc::Control,
    }),
    rdram_interface: Peripheral::new(ri::RdramInterface {
        current_load: ri::CurrentLoad,
        write_error: ri::WriteError,
        read_error: ri::ReadError,
        latency: ri::Latency,
        refresh: ri::Refresh,
        config: ri::Config,
        select: ri::Select,
        mode: ri::Mode,
    }),
    video_interface: Peripheral::new(vi::VideoInterface),
    mips_interface: Peripheral::new(mi::MipsInterface {
        interrupt_masks: mi::InterruptMasks,
        interrupts: mi::Interrupts,
        version: mi::Version,
        mode: mi::Mode,
    }),
    dps: Peripheral::new(dps::Dps {
        buffer_test_address: dps::BufferTestAddress,
        buffer_test_data: dps::BufferTestData,
        buffer_test_mode: dps::BufferTestMode,
        tmem_bist: dps::TmemBist,
    }),
};

/// Contains references to each hardware peripheral on the system.
pub struct Hardware {
    /// Controlled reference to the peripheral interface.
    pub peripheral_interface: Peripheral<pi::PeripheralInterface>,

    /// Controlled reference to the serial interface.
    pub serial_interface: Peripheral<si::SerialInterface>,

    /// Controlled reference to the program counter.
    pub program_counter: Peripheral<pc::ProgramCounter>,

    /// Controlled reference to the RDRAM interface.
    pub rdram_interface: Peripheral<ri::RdramInterface>,

    /// Controlled reference to the video interface.
    pub video_interface: Peripheral<vi::VideoInterface>,

    /// Controlled reference to the MIPS interface.
    pub mips_interface: Peripheral<mi::MipsInterface>,

    /// Controlled reference to the DPS system.
    pub dps: Peripheral<dps::Dps>,
}

/// An ownership wrapper for peripherals.
///
/// ```rust
/// # use nintendo64_pac::{HardwareError, Peripheral};
/// let mut foo = Peripheral::new(12345);
///
/// let bar = foo.take(); // Ok(12345)
/// # assert!(bar.is_ok());
/// let baz = foo.take(); // Err(HardwareError::TakePeripheralError)
/// # assert!(baz.is_err());
/// ```
pub struct Peripheral<T>(Option<T>);

impl<T> Peripheral<T> {
    /// Creates a new wrapper around the given peripheral.
    pub const fn new(peripheral: T) -> Self {
        Self(Some(peripheral))
    }

    /// Surrenders ownership of this peripheral to the caller.
    pub fn take(&mut self) -> Result<T, HardwareError> {
        replace(&mut self.0, None).ok_or(HardwareError::TakePeripheralError)
    }

    /// Takes ownership of the given peripheral back from the caller.
    pub fn drop(&mut self, peripheral: T) {
        self.0 = Some(peripheral);
    }
}

/// Errors related to the [`Hardware`][crate::Hardware] mechanism.
pub enum HardwareError {
    /// Occurs when an attempt is made to take a peripheral which has already
    /// been taken.
    TakePeripheralError,
}

#[macro_export]
macro_rules! register_access {
    ($base:expr, $reg_type:tt) => {
        const REGS_BASE: usize = $base;

        fn registers<'a>() -> &'a $reg_type {
            unsafe { &*(REGS_BASE as *const $reg_type) }
        }

        unsafe impl Sync for $reg_type {}
    };
}
