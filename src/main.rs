#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::ptr::write_volatile;

// =============================================================================
// 1. Startup and External Symbols
// =============================================================================

extern "C" {
    // This is the entry point defined in trap.S
    fn __trap_vector();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Manually set the stack pointer to a safe area in QEMU virt RAM.
    unsafe {
        asm!("li sp, 0x80100000");
    }

    // Initialize UART first for diagnostics
    uart_puts("corode-core booting...\n");
    
    // 2. Setup Trap Vector
    setup_trap_vector();
    uart_puts("Trap-Handler 'Harlekin' an mtvec gebunden.\n");

    // 3. Setup PMP Shield
    setup_pmp_shield();
    uart_puts("PMP-Schild aktiv (Vault isoliert).\n");

    // 4. Intentional Self-Attack (The "Eierschalensollbruchstellenverursacher")
    uart_puts("\n>>> PROVOZIERE SELBSTANGRIFF AUF PMP-VAULT <<<\n");
    
    let vault_ptr = VAULT_START as *mut u32;
    unsafe {
        // This write to a read-only, locked PMP region MUST trigger a trap.
        write_volatile(vault_ptr, 0x1337);
    }

    // This line should never be reached if PMP and traps are working.
    uart_puts("!!! FEHLER: Selbstangriff wurde nicht abgefangen! !!!\n");

    loop {
        unsafe { asm!("wfi"); }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    uart_puts("\n!!! KERNEL PANIC !!!\n");
    loop {
        unsafe { asm!("wfi"); }
    }
}

// =============================================================================
// 2. Minimal UART (QEMU virt)
// =============================================================================
const UART_BASE: *mut u8 = 0x10000000 as *mut u8;

fn uart_putc(c: u8) {
    unsafe { write_volatile(UART_BASE, c); }
}

fn uart_puts(s: &str) {
    for byte in s.bytes() { uart_putc(byte); }
}

fn uart_puthex(mut n: usize) {
    uart_puts("0x");
    let hex_digits = b"0123456789abcdef";
    let mut buffer = [0u8; 16];
    let mut i = 16;

    if n == 0 {
        uart_putc(b'0');
        return;
    }

    while n > 0 {
        i -= 1;
        buffer[i] = hex_digits[n & 0xf];
        n >>= 4;
    }

    for &c in &buffer[i..] {
        uart_putc(c);
    }
}

// =============================================================================
// 3. PMP (Physical Memory Protection)
// =============================================================================

// Define a 4KB Vault. NAPOT requires alignment to size.
#[repr(align(4096))]
struct Vault([u8; 4096]);

#[link_section = ".vault"]
static VAULT_STORAGE: Vault = Vault([0; 4096]);

const VAULT_START: usize = &VAULT_STORAGE as *const Vault as usize;
const VAULT_SIZE: usize = 4096;

fn setup_pmp_shield() {
    // NAPOT (Naturally Aligned Power-of-Two) calculation:
    // The pmpaddr register holds the range encoded as: (base >> 2) | ((size - 1) >> 3)
    let pmp_addr_val = (VAULT_START >> 2) | ((VAULT_SIZE - 1) >> 3);

    const PMP_R: u8 = 1 << 0;
    const PMP_A_NAPOT: u8 = 3 << 3;
    const PMP_L: u8 = 1 << 7;

    // Config: Read-only (R), NAPOT mode, Locked (L)
    let pmp_cfg_val: u8 = PMP_R | PMP_A_NAPOT | PMP_L;

    unsafe {
        asm!(
            "csrw pmpaddr0, {addr}",
            "csrw pmpcfg0, {cfg}",
            addr = in(reg) pmp_addr_val,
            cfg = in(reg) pmp_cfg_val,
        );
    }
}

// =============================================================================
// 4. Trap Handling
// =============================================================================

fn setup_trap_vector() {
    unsafe {
        // Point mtvec to the assembly entry point in trap.S
        asm!("csrw mtvec, {}", in(reg) __trap_vector as usize);
    }
}

/// The Rust side of the trap handler, called by trap.S
#[no_mangle]
pub unsafe extern "C" fn rust_handle_trap() -> ! {
    let mcause: usize;
    let mepc: usize;
    let mtval: usize;
    
    asm!("csrr {}, mcause", out(reg) mcause);
    asm!("csrr {}, mepc", out(reg) mepc);
    asm!("csrr {}, mtval", out(reg) mtval);

    // Check if it's an interrupt (top bit set)
    let is_interrupt = (mcause >> (core::mem::size_of::<usize>() * 8 - 1)) != 0;
    let cause_code = mcause & !(1usize << (core::mem::size_of::<usize>() * 8 - 1));

    uart_puts("\n**********************************************\n");
    uart_puts(">> Harlekin says NEIN, Bro. <<\n");
    uart_puts("**********************************************\n");
    
    if is_interrupt {
        uart_puts("Interrupt erkannt: ");
    } else {
        uart_puts("Exception erkannt: ");
    }
    uart_puthex(cause_code);
    
    match (is_interrupt, cause_code) {
        (false, 7) => uart_puts(" (Store access fault) -> PMP ERFOLGREICH!\n"),
        (false, 5) => uart_puts(" (Load access fault)\n"),
        _ => uart_puts(" (Unbekannte Ursache)\n"),
    }
    
    uart_puts("  mepc:  "); uart_puthex(mepc); uart_puts("\n");
    uart_puts("  mtval: "); uart_puthex(mtval); uart_puts("\n");
    uart_puts("**********************************************\n");
    uart_puts("System eingefroren. Ordnung wiederhergestellt.\n");

    loop {
        asm!("wfi");
    }
}
