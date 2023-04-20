use tock_registers::{register_bitfields, register_structs, registers::ReadWrite};

const PC_REGS_BASE: usize = 0x0408_0000;

#[non_exhaustive]
pub struct ProgramCounter;

impl ProgramCounter {
    fn registers<'a>(&self) -> &'a ProgramCounterRegisters {
        unsafe { &mut *(PC_REGS_BASE as *mut ProgramCounterRegisters) }
    }
}

register_structs! {
    ProgramCounterRegisters {
        (0x0000 => pub program_counter: ReadWrite<u32, ProgramCounterControl::Register>),
        (0x0004 => pub imem_bist: ReadWrite<u32, ImemBist::Register>),
        (0x0008 => @END),
    }
}

register_bitfields! {
    u32,

    ProgramCounterControl [
        PROGRAM_COUNTER          OFFSET(0)  NUMBITS(12) [],
    ],

    ImemBist [
        BIST_CHECK               OFFSET(0)  NUMBITS(1)  [],
        BIST_GO                  OFFSET(1)  NUMBITS(1)  [],
        BIST_CLEAR               OFFSET(2)  NUMBITS(1)  [],
        BIST_DONE                OFFSET(2)  NUMBITS(1)  [],
        BIST_FAIL                OFFSET(3)  NUMBITS(4)  [],
    ]
}
