//! # Nintendo 64 PAC

#![no_std]

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
    pub use super::dps::Dps;
    pub use super::mi::Mi;
    pub use super::pi::Pi;
    pub use super::rdram::Rdram;
    pub use super::ri::Ri;
    pub use super::sp::Sp;
    pub use super::vi::Vi;
}
