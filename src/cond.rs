#![allow(dead_code)] // Temp: Bis alles verdrahtet ist

#[derive(Clone, Copy, Debug)]
pub struct CondMask(pub u64);

impl CondMask {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn from_bit(bit: u8) -> Self {
        Self(1u64 << bit)
    }

    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    pub const fn contains_all(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }

    pub const fn contains_any(self, other: Self) -> bool {
        (self.0 & other.0) != 0
    }

    pub fn set(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn clear(&mut self, other: Self) {
        self.0 &= !other.0;
    }
}

pub mod id {
    pub const BOOT_DONE: u8     = 0;
    pub const UART_RX_READY: u8 = 1;
    pub const TIMER_TICK: u8    = 2;
    pub const FAULT_SEEN: u8    = 3;
    pub const PMP_OK: u8        = 4;
    pub const HEARTBEAT: u8     = 5;
}
