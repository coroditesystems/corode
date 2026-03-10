#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::ptr::write_volatile;

// =============================================================================
// 1. Startup and Linker Symbols
// =============================================================================

// Extern "C" to link against the assembly trap entry point.
extern "C" {
    fn __trap_vector();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Manually set the stack pointer. For QEMU 'virt', RAM ends around 0x88000000.
    // We set the SP to the end of RAM, growing downwards.
    unsafe {
        asm!("li sp, 0x88000000");
    }

    // Initialize all subsystems
    uart_puts("UART initialized.\n");
    
    setup_trap_vector();
    uart_puts("Trap vector configured.\n");

    setup_pmp();
    uart_puts("PMP vault configured and locked.\n");

    // Announce and perform the self-attack test
    uart_puts("\n>>> INITIATING SELF-ATTACK ON READ-ONLY VAULT <<<\n");

    // This volatile write to a read-only, PMP-protected region
    // will trigger a store access fault.
    let vault_ptr = VAULT_START as *mut u32;
    unsafe {
        write_volatile(vault_ptr, 0xDEADBEEF);
    }

    // If we ever reach this point, the PMP protection or the trap failed.
    uart_puts("\n!!! TEST FAILED: Trap was not triggered. !!!\n");

    // Halt the core
    loop {
        unsafe { asm!("wfi"); }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart_puts("\n! KERNEL PANIC !\n");
    if let Some(s) = info.payload().downcast_ref::<&str>() {
        uart_puts(s);
        uart_puts("\n");
    }
    // Halt
    loop {
        unsafe { asm!("wfi"); }
    }
}


// =============================================================================
// 2. Minimal UART Helpers for QEMU
// =============================================================================
const UART_BASE: *mut u8 = 0x10000000 as *mut u8;

fn uart_putc(c: u8) {
    unsafe {
        write_volatile(UART_BASE, c);
    }
}

fn uart_puts(s: &str) {
    for byte in s.bytes() {
        uart_putc(byte);
    }
}

fn uart_puthex(mut n: usize) {
    uart_puts("0x");
    if n == 0 {
        uart_putc('0');
        return;
    }
    let hex_digits = b"0123456789abcdef";
    let mut buffer = [0u8; 16];
    let mut i = 15;
    while n > 0 {
        buffer[i] = hex_digits[n % 16];
        n /= 16;
        i -= 1;
    }
    for &c in &buffer[i+1..] {
        uart_putc(c);
    }
}


// =============================================================================
// 3. PMP Configuration (Physical Memory Protection)
// =============================================================================

// Define a 4 KiB vault region, aligned to its own size to satisfy NAPOT requirements.
#[repr(align(4096))]
struct Vault([u8; 4096]);
static VAULT_STORAGE: Vault = Vault([0; 4096]);

const VAULT_START: usize = &VAULT_STORAGE as *const Vault as usize;
const VAULT_SIZE: usize = 4096;

fn setup_pmp() {
    // NAPOT (Naturally Aligned Power-of-Two) is the cleanest way to protect a
    // region. It uses a single PMP entry.
    // The pmpaddr is calculated from the base address and size.
    let pmp_addr_val = (VAULT_START >> 2) | ((VAULT_SIZE - 1) >> 3);

    // PMP Configuration Flags
    const PMP_R: u8 = 1 << 0; // Read
    const PMP_A_NAPOT: u8 = 3 << 3; // NAPOT addressing mode
    const PMP_L: u8 = 1 << 7; // Locked

    // We configure the region as Read-Only and Locked.
    // Any write or modification of the entry will now cause a fault.
    let pmp_cfg_val: u8 = PMP_R | PMP_A_NAPOT | PMP_L;

    unsafe {
        asm!(
            "csrw pmpaddr0, {pmp_addr}",
            "csrw pmpcfg0, {pmp_cfg}",
            pmp_addr = in(reg) pmp_addr_val,
            pmp_cfg = in(reg) pmp_cfg_val,
        );
    }
}


// =============================================================================
// 4. Trap Handling
// =============================================================================

fn setup_trap_vector() {
    unsafe {
        // Set mtvec to our assembly stub. Direct mode.
        asm!("csrw mtvec, {}", in(reg) __trap_vector as usize);
    }
}

/// This is the Rust function called from the assembly trap stub.
#[no_mangle]
pub extern "C" fn rust_handle_trap(_trap_frame: *const usize) {
    let mcause: usize;
    let mepc: usize;
    let mtval: usize;
    unsafe {
        asm!("csrr {}, mcause", out(reg) mcause);
        asm!("csrr {}, mepc", out(reg) mepc);
        asm!("csrr {}, mtval", out(reg) mtval);
    }

    // Harlekin's verdict, with technical details.
    uart_puts("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
    uart_puts(">>> Harlekin says NEIN, Bro. <<<\n");
    uart_puts("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
    uart_puts("Caught an exception! Details:\n");

    uart_puts("  MCAUSE: ");
    uart_puthex(mcause);
    match mcause {
        5 => uart_puts(" (Load access fault)\n"),
        7 => uart_puts(" (Store/AMO access fault) -> CORRECT!\n"),
        _ => uart_puts(" (Unknown cause)\n"),
    }

    uart_puts("  MEPC (Instruction Address): ");
    uart_puthex(mepc);
    uart_puts("\n");

    uart_puts("  MTVAL (Faulting Address):   ");
    uart_puthex(mtval);
    uart_puts("\n\nSystem is now halting.\n");

    // Halt forever
    loop {
        unsafe { asm!("wfi"); }
    }
}
