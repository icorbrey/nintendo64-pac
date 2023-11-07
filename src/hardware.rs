use crate::prelude::{Ai, Dpc, Dps, Mi, Pc, Pi, Rdram, Ri, Si, Sp, Vi};

pub struct Hardware {
    pub ai: Ai,
    pub dpc: Dpc,
    pub dps: Dps,
    pub mi: Mi,
    pub pc: Pc,
    pub pi: Pi,
    pub rdram: Rdram,
    pub ri: Ri,
    pub si: Si,
    pub sp: Sp,
    pub vi: Vi,
}

static mut TAKEN: bool = false;

impl Hardware {
    pub unsafe fn new() -> Self {
        TAKEN = true;
        Self {
            ai: Ai::new(),
            dpc: Dpc::new(),
            dps: Dps::new(),
            mi: Mi::new(),
            pc: Pc::new(),
            rdram: Rdram::new(),
            pi: Pi::new(),
            ri: Ri::new(),
            si: Si::new(),
            sp: Sp::new(),
            vi: Vi::new(),
        }
    }

    pub fn take() -> Option<Self> {
        if unsafe { !TAKEN } {
            Some(unsafe { Self::new() })
        } else {
            None
        }
    }
}
