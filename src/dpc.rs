use tock_registers::{
    interfaces::{Readable, Writeable},
    register_bitfields, register_structs,
    registers::{ReadOnly, ReadWrite},
};

const DPC_REGS_BASE: usize = 0x0410_0000;

#[non_exhaustive]
pub struct Dpc;

impl Dpc {
    fn registers<'a>(&self) -> &'a DpcRegisters {
        unsafe { &mut *(DPC_REGS_BASE as *mut DpcRegisters) }
    }

    pub fn dma_start(&self) -> u32 {
        self.registers().dma_start.read(DmaAddress::ADDRESS)
    }

    pub fn set_dma_start(&self, dma_start: u32) {
        self.registers()
            .dma_start
            .write(DmaAddress::ADDRESS.val(dma_start));
    }

    pub fn dma_end(&self) -> u32 {
        self.registers().dma_end.read(DmaAddress::ADDRESS)
    }

    pub fn set_dma_end(&self, dma_end: u32) {
        self.registers()
            .dma_end
            .write(DmaAddress::ADDRESS.val(dma_end));
    }

    pub fn dma_current(&self) -> u32 {
        self.registers().dma_current.read(DmaAddress::ADDRESS)
    }

    pub fn set_dma_current(&self, dma_current: u32) {
        self.registers()
            .dma_current
            .write(DmaAddress::ADDRESS.val(dma_current));
    }

    pub fn status(&self) -> DpcStatus {
        let registers = self.registers();
        DpcStatus {
            xbus_dmem_dma: registers.status.is_set(Status::XBUS_DMEM_DMA),
            freeze: registers.status.is_set(Status::FREEZE),
            flush: registers.status.is_set(Status::FLUSH),
            start_glck: registers.status.is_set(Status::START_GLCK),
            is_texture_memory_busy: registers.status.is_set(Status::TMEM_BUSY),
            is_pipe_busy: registers.status.is_set(Status::PIPE_BUSY),
            is_command_busy: registers.status.is_set(Status::CMD_BUSY),
            is_cbuf_ready: registers.status.is_set(Status::CBUF_READY),
            is_dma_busy: registers.status.is_set(Status::DMA_BUSY),
            is_end_valid: registers.status.is_set(Status::END_VALID),
            is_start_valid: registers.status.is_set(Status::START_VALID),
        }
    }

    pub fn set_status(&self, status: DpcStatus) {
        self.registers().status.write(
            Status::XBUS_DMEM_DMA.val(status.xbus_dmem_dma.into())
                + Status::FREEZE.val(status.freeze.into())
                + Status::FLUSH.val(status.flush.into())
                + Status::START_GLCK.val(status.start_glck.into())
                + Status::TMEM_BUSY.val(status.is_texture_memory_busy.into())
                + Status::PIPE_BUSY.val(status.is_pipe_busy.into())
                + Status::CMD_BUSY.val(status.is_command_busy.into())
                + Status::CBUF_READY.val(status.is_cbuf_ready.into())
                + Status::DMA_BUSY.val(status.is_dma_busy.into())
                + Status::END_VALID.val(status.is_end_valid.into())
                + Status::START_VALID.val(status.is_start_valid.into()),
        )
    }

    pub fn clock(&self) -> u32 {
        self.registers().clock.read(ClockCounter::CLOCK_COUNTER)
    }

    pub fn buffer_busy(&self) -> u32 {
        self.registers()
            .buffer_busy
            .read(ClockCounter::CLOCK_COUNTER)
    }

    pub fn pipe_busy(&self) -> u32 {
        self.registers().pipe_busy.read(ClockCounter::CLOCK_COUNTER)
    }

    pub fn texture_memory(&self) -> u32 {
        self.registers()
            .texture_memory
            .read(ClockCounter::CLOCK_COUNTER)
    }
}

pub struct DpcStatus {
    pub xbus_dmem_dma: bool,
    pub freeze: bool,
    pub flush: bool,
    pub start_glck: bool,
    pub is_texture_memory_busy: bool,
    pub is_pipe_busy: bool,
    pub is_command_busy: bool,
    pub is_cbuf_ready: bool,
    pub is_dma_busy: bool,
    pub is_end_valid: bool,
    pub is_start_valid: bool,
}

register_structs! {
    DpcRegisters {
        (0x0000 => pub dma_start: ReadWrite<u32, DmaAddress::Register>),
        (0x0004 => pub dma_end: ReadWrite<u32, DmaAddress::Register>),
        (0x0008 => pub dma_current: ReadWrite<u32, DmaAddress::Register>),
        (0x000C => pub status: ReadWrite<u32, Status::Register>),
        (0x0010 => pub clock: ReadOnly<u32, ClockCounter::Register>),
        (0x0014 => pub buffer_busy: ReadOnly<u32, ClockCounter::Register>),
        (0x0018 => pub pipe_busy: ReadOnly<u32, ClockCounter::Register>),
        (0x001C => pub texture_memory: ReadOnly<u32, ClockCounter::Register>),
        (0x0020 => @END),
    }
}

register_bitfields! {
    u32,

    DmaAddress [
        ADDRESS       OFFSET(0)  NUMBITS(24) [],
    ],

    Status [
        XBUS_DMEM_DMA OFFSET(0)  NUMBITS(1)  [],
        FREEZE        OFFSET(1)  NUMBITS(1)  [],
        FLUSH         OFFSET(2)  NUMBITS(1)  [],
        START_GLCK    OFFSET(3)  NUMBITS(1)  [],
        TMEM_BUSY     OFFSET(4)  NUMBITS(1)  [],
        PIPE_BUSY     OFFSET(5)  NUMBITS(1)  [],
        CMD_BUSY      OFFSET(6)  NUMBITS(1)  [],
        CBUF_READY    OFFSET(7)  NUMBITS(1)  [],
        DMA_BUSY      OFFSET(8)  NUMBITS(1)  [],
        END_VALID     OFFSET(9)  NUMBITS(1)  [],
        START_VALID   OFFSET(10) NUMBITS(1)  [],
    ],

    ClockCounter [
        CLOCK_COUNTER OFFSET(0)  NUMBITS(24) [],
    ]
}
