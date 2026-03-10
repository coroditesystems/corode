use crate::cond::CondMask;

// Globaler Zustand für Pending Conditions von Traps
static mut PENDING_CONDS: u64 = 0;

pub fn pend_cond(bit: u8) {
    // Sicher, weil wir nur atomare Operationen auf einem einzigen u64 ausführen
    // und der RISC-V Store im Normalfall atomar ist.
    unsafe {
        PENDING_CONDS |= 1u64 << bit;
    }
}

pub struct CoreState {
    pub conds: CondMask,
    pub epoch: u64,
}

impl CoreState {
    pub const fn new() -> Self {
        Self {
            conds: CondMask::empty(),
            epoch: 0,
        }
    }

    pub fn set_cond(&mut self, bit: u8) {
        self.conds.set(CondMask::from_bit(bit));
    }

    pub fn drain_pending_conds(&mut self) {
        let pending = unsafe {
            let p = PENDING_CONDS;
            PENDING_CONDS = 0;
            p
        };
        self.conds.set(CondMask(pending));
    }
}
