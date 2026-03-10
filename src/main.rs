#![no_std]
#![no_main]
#![feature(const_fn_trait_bound)]

// =============================================================================
// ⚙️ corode-core: src/main.rs
// =============================================================================

use core::panic::PanicInfo;
use core::arch::asm;

// =============================================================================
// I. Die Verfassung: PMP-Konstanten & Adress-Layout
// =============================================================================
const PMP_R: u8 = 1 << 0; const PMP_W: u8 = 1 << 1; const PMP_X: u8 = 1 << 2;
const PMP_A_OFF: u8 = 0; const PMP_A_TOR: u8 = 1; const PMP_A_NA4: u8 = 2; const PMP_A_NAPOT: u8 = 3;
const PMP_L: u8 = 1 << 7;

// Adress-Layout
const ADDR_PMP_HEADER_CAGE: usize = 0x8000F000;   // 4KB Käfig für die PMP-Konfiguration selbst
const ADDR_VEC_0_THRON: usize = 0x80000000;      // 64KB Kernel
const ADDR_VEC_1_WAECHTER: usize = 0x80010000;   // 4KB Harlekin
const ADDR_VEC_2_SCHREIBER: usize = 0x10000000;   // 1MB Logger
const ADDR_VEC_2_SCHREIBER_END: usize = ADDR_VEC_2_SCHREIBER + (1024 * 1024) -1;
const ADDR_VEC_3_ORAKEL: usize = 0x10010000;    // 1MB Z3
const ADDR_VEC_5_EXIL: usize = 0x90000000;       // 256MB Turing-Band
const ADDR_VEC_6_BOTE: usize = 0x80020000;       // 64KB corode-net
const ADDR_VEC_7_AUGE: usize = 0xA0000000;       // 32MB corode-ui Framebuffer
const ADDR_VEC_8_ARENA0: usize = 0xB0000000;     // 128MB Condition-Sandkiste
const ADDR_VEC_9_LEHRER: usize = 0xC0000000;     // 1GB AI Sidekernel
const ADDR_VEC_10_TREIBER: usize = 0xD0000000;    // 1GB Linux Sidekernel
const ADDR_VEC_11_DIPLOMAT: usize = 0xE0000000;   // 1GB Unix Sidekernel
const ADDR_VEC_14_WERKZEUGE: usize = 0x0C000000; // MMIO Bereich
const ADDR_4_BIT_BREAKER: *mut u32 = (ADDR_VEC_14_WERKZEUGE + 0x1000) as *mut u32;
const ADDR_VEC_15_ZIEL: usize = 0xDEADBEEF;     // Einzelne, gesperrte Adresse


// =============================================================================
// II. Die PMP Vektor Map: Datengetriebene Sicherheitskonfiguration
// =============================================================================

#[repr(C)]
struct PmpEntry {
    addr: usize,
    size: usize,
    permissions: u8,
}

#[link_section = ".pmp_cage"]
static PMP_VECTOR_MAP: [PmpEntry; 16] = [
    PmpEntry { addr: ADDR_VEC_0_THRON, size: 64 * 1024, permissions: PMP_R | PMP_X | PMP_L },
    PmpEntry { addr: ADDR_VEC_1_WAECHTER, size: 4 * 1024, permissions: PMP_R | PMP_X | PMP_L },
    PmpEntry { addr: ADDR_VEC_2_SCHREIBER, size: 1 * 1024 * 1024, permissions: PMP_R | PMP_W | PMP_L },
    PmpEntry { addr: ADDR_VEC_3_ORAKEL, size: 1 * 1024 * 1024, permissions: PMP_R | PMP_X | PMP_L },
    PmpEntry { addr: ADDR_PMP_HEADER_CAGE, size: 4 * 1024, permissions: PMP_R | PMP_L },
    PmpEntry { addr: ADDR_VEC_5_EXIL, size: 256 * 1024 * 1024, permissions: PMP_R | PMP_W | PMP_L },
    PmpEntry { addr: ADDR_VEC_6_BOTE, size: 64 * 1024, permissions: PMP_R | PMP_W | PMP_X | PMP_L },
    PmpEntry { addr: ADDR_VEC_7_AUGE, size: 32 * 1024 * 1024, permissions: PMP_R | PMP_W | PMP_L },
    PmpEntry { addr: ADDR_VEC_8_ARENA0, size: 128 * 1024 * 1024, permissions: PMP_A_NAPOT },
    PmpEntry { addr: ADDR_VEC_9_LEHRER, size: 1024 * 1024 * 1024, permissions: PMP_R | PMP_W | PMP_X },
    PmpEntry { addr: ADDR_VEC_10_TREIBER, size: 1024 * 1024 * 1024, permissions: PMP_R | PMP_W | PMP_X },
    PmpEntry { addr: ADDR_VEC_11_DIPLOMAT, size: 1024 * 1024 * 1024, permissions: PMP_R | PMP_W | PMP_X },
    PmpEntry { addr: 0, size: 0, permissions: 0 },
    PmpEntry { addr: 0, size: 0, permissions: 0 },
    PmpEntry { addr: ADDR_VEC_14_WERKZEUGE, size: 16 * 1024 * 1024, permissions: PMP_R | PMP_W },
    PmpEntry { addr: ADDR_VEC_15_ZIEL, size: 4, permissions: PMP_R | PMP_L },
];


// =============================================================================
// III. Der Chronist: Trickster (Globale Singleton Instanz)
// =============================================================================
struct Trickster {
    cursor: *mut u8,
    start_addr: usize,
    end_addr: usize,
}

static mut GLOBAL_TRICKSTER: Trickster = Trickster {
    cursor: ADDR_VEC_2_SCHREIBER as *mut u8,
    start_addr: ADDR_VEC_2_SCHREIBER,
    end_addr: ADDR_VEC_2_SCHREIBER_END,
};

impl Trickster {
    fn get_global() -> &'static mut Trickster {
        unsafe { &mut GLOBAL_TRICKSTER }
    }
    
    fn log_byte(&mut self, byte: u8) {
        unsafe {
            if self.cursor as usize >= self.end_addr {
                self.cursor = self.start_addr as *mut u8;
            }
            core::ptr::write_volatile(self.cursor, byte);
            self.cursor = self.cursor.add(1);
        }
    }

    fn log(&mut self, message: &str) {
        for &byte in message.as_bytes() {
            self.log_byte(byte);
        }
    }
    
    fn log_hex(&mut self, n: usize) {
        let mut temp = n;
        let mut buffer = [0u8; 16];
        let mut i = 15;
        self.log("0x");
        if temp == 0 { self.log_byte(b'0'); return; }
        loop {
            let digit = (temp % 16) as u8;
            buffer[i] = if digit < 10 { digit + b'0' } else { digit - 10 + b'a' };
            temp /= 16;
            if temp == 0 { break; }
            i -= 1;
        }
        for &byte in &buffer[i..] {
            self.log_byte(byte);
        }
    }
}

// =============================================================================
// IV. Der letzte Ausweg: Panic Handler
// =============================================================================
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! { loop {} }

// =============================================================================
// V. Der Souverän: _start (Eintrittspunkt)
// =============================================================================

struct Lcg { state: u32 }
impl Lcg {
    fn new(seed: u32) -> Self { Lcg { state: seed } }
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(1103515245).wrapping_add(12345);
        self.state
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    setup_pmp_vectors();
    let trickster = Trickster::get_global();
    asm!("csrw mtvec, {}", in(reg) harlekin_trap_handler as usize);
    
    let mut rng = Lcg::new(0xCAFEF00D); 
    let random_value = rng.next();

    if (random_value >> 16) == 0xDEAD {
        trickster.log("CHAOS-ANOMALIE ENTDECKT! \n");
        trickster.log("INITIIERE OOC-SELBSTANGRIFF...\n");
        core::ptr::write_volatile(ADDR_VEC_15_ZIEL as *mut u32, 0x1337);
    } else {
        trickster.log("PMP-Schilde oben. Selbsttest ohne Anomalie. System stabil.\n");
    }

    loop {}
}

// =============================================================================
// VI. Der Wächter: Harlekin Trap Handler
// =============================================================================
#[no_mangle]
#[repr(align(4))]
pub unsafe extern "C" fn harlekin_trap_handler() {
    let trickster = Trickster::get_global();
    let (mcause, mepc, mtval): (usize, usize, usize);
    asm!("csrr {}, mcause", out(reg) mcause);
    asm!("csrr {}, mepc", out(reg) mepc);
    asm!("csrr {}, mtval", out(reg) mtval);

    trickster.log("\n>> HARLEKIN FÄNGT EINDRINGLING AB! <<\n");
    trickster.log("   URSACHE: "); trickster.log_hex(mcause);
    match mcause {
        5 => trickster.log(" (Load access fault)\n"),
        7 => trickster.log(" (Store/AMO access fault)\n"),
        _ => trickster.log(" (Unknown trap cause)\n"),
    }
    trickster.log("   ORT:     "); trickster.log_hex(mepc);
    trickster.log("\n   ZIEL:    "); trickster.log_hex(mtval);
    trickster.log("\n");

    asm!("csrw mepc, {}", in(reg) (mepc + 4));
}

// =============================================================================
// VII. Die Schmiede: PMP Vektor-Setup & 4-Bit Breaker
// =============================================================================

// Schreibt den komprimierten PMP-Lock-Status an den simulierten Hardware-Port.
unsafe fn update_4_bit_breaker(status: u32) {
    core::ptr::write_volatile(ADDR_4_BIT_BREAKER, status);
}

#[inline(never)]
unsafe fn setup_pmp_vectors() {
    for (i, entry) in PMP_VECTOR_MAP.iter().enumerate() {
        let pmp_addr = (entry.addr >> 2) | ((entry.size.wrapping_sub(1)) >> 3);
        let pmp_cfg = if entry.size > 0 {
            (entry.permissions) | (PMP_A_NAPOT << 3)
        } else {
            0
        };

        match i {
            0 => { asm!("csrw pmpaddr0, {}", in(reg) pmp_addr); asm!("csrw pmpcfg0, {}", in(reg) pmp_cfg); },
            1 => { asm!("csrw pmpaddr1, {}", in(reg) pmp_addr); asm!("csrw pmpcfg1, {}", in(reg) pmp_cfg); },
            2 => { asm!("csrw pmpaddr2, {}", in(reg) pmp_addr); asm!("csrw pmpcfg2, {}", in(reg) pmp_cfg); },
            3 => { asm!("csrw pmpaddr3, {}", in(reg) pmp_addr); asm!("csrw pmpcfg3, {}", in(reg) pmp_cfg); },
            4 => { asm!("csrw pmpaddr4, {}", in(reg) pmp_addr); asm!("csrw pmpcfg4, {}", in(reg) pmp_cfg); },
            5 => { asm!("csrw pmpaddr5, {}", in(reg) pmp_addr); asm!("csrw pmpcfg5, {}", in(reg) pmp_cfg); },
            6 => { asm!("csrw pmpaddr6, {}", in(reg) pmp_addr); asm!("csrw pmpcfg6, {}", in(reg) pmp_cfg); },
            7 => { asm!("csrw pmpaddr7, {}", in(reg) pmp_addr); asm!("csrw pmpcfg7, {}", in(reg) pmp_cfg); },
            8 => { asm!("csrw pmpaddr8, {}", in(reg) pmp_addr); asm!("csrw pmpcfg8, {}", in(reg) pmp_cfg); },
            9 => { asm!("csrw pmpaddr9, {}", in(reg) pmp_addr); asm!("csrw pmpcfg9, {}", in(reg) pmp_cfg); },
            10 => { asm!("csrw pmpaddr10, {}", in(reg) pmp_addr); asm!("csrw pmpcfg10, {}", in(reg) pmp_cfg); },
            11 => { asm!("csrw pmpaddr11, {}", in(reg) pmp_addr); asm!("csrw pmpcfg11, {}", in(reg) pmp_cfg); },
            12 => { asm!("csrw pmpaddr12, {}", in(reg) pmp_addr); asm!("csrw pmpcfg12, {}", in(reg) pmp_cfg); },
            13 => { asm!("csrw pmpaddr13, {}", in(reg) pmp_addr); asm!("csrw pmpcfg13, {}", in(reg) pmp_cfg); },
            14 => { asm!("csrw pmpaddr14, {}", in(reg) pmp_addr); asm!("csrw pmpcfg14, {}", in(reg) pmp_cfg); },
            15 => { asm!("csrw pmpaddr15, {}", in(reg) pmp_addr); asm!("csrw pmpcfg15, {}", in(reg) pmp_cfg); },
            _ => {},
        }
    }

    // Berechne und setze den 4-Bit Breaker Status.
    let mut lock_bits: u16 = 0;
    for (i, entry) in PMP_VECTOR_MAP.iter().enumerate() {
        if (entry.permissions & PMP_L) != 0 {
            lock_bits |= 1 << i;
        }
    }
    
    // XOR-Folding der 16 Lock-Bits zu einem 4-Bit-Wert.
    let folded = ((lock_bits >> 12) & 0xF) ^ ((lock_bits >> 8) & 0xF) ^ ((lock_bits >> 4) & 0xF) ^ (lock_bits & 0xF);
    update_4_bit_breaker(folded as u32);
}
