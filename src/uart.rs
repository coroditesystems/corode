use core::ptr::{read_volatile, write_volatile};

// UART-Registeradressen für das SiFive HiFive1 Board (FE310-G002)
const UART0_BASE: usize = 0x10020000;
const UART0_TXDATA: *mut u32 = (UART0_BASE + 0x00) as *mut u32;
const UART0_RXDATA: *const u32 = (UART0_BASE + 0x04) as *const u32;

// ANSI Escape Codes für Farben
const PINK_COLOR: &'static str = "\x1b[95m";
const RESET_COLOR: &'static str = "\x1b[0m";
static mut PINK_MODE_ENABLED: bool = false;

fn tx_fifo_full() -> bool {
    // Das MSB des TXDATA-Registers zeigt an, ob der FIFO voll ist
    unsafe { read_volatile(UART0_TXDATA) & 0x80000000 != 0 }
}

fn uart_putc(c: u8) {
    // Warten, bis im FIFO Platz ist
    while tx_fifo_full() {}
    unsafe { write_volatile(UART0_TXDATA, c as u32) };
}

pub fn uart_puts(s: &str) {
    unsafe {
        if PINK_MODE_ENABLED {
            for byte in PINK_COLOR.as_bytes() {
                uart_putc(*byte);
            }
        }
    }

    for byte in s.as_bytes() {
        uart_putc(*byte);
    }

    unsafe {
        if PINK_MODE_ENABLED {
            for byte in RESET_COLOR.as_bytes() {
                uart_putc(*byte);
            }
        }
    }
}

pub fn uart_getc() -> Option<u8> {
    let rxdata = unsafe { read_volatile(UART0_RXDATA) };
    if rxdata & 0x80000000 == 0 {
        // Das LSB des RXDATA-Registers enthält das empfangene Byte
        Some((rxdata & 0xFF) as u8)
    } else {
        None
    }
}

pub fn set_pink_mode(enabled: bool) {
    unsafe {
        PINK_MODE_ENABLED = enabled;
    }
}
