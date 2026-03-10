#![no_std]
#![no_main]

mod pmp;
mod uart;
mod trickster;
mod zuse;
mod sidekernel;
mod condition;
mod terminal;
mod commands;

use core::arch::asm;
use crate::sidekernel::Sidekernel;
use crate::terminal::TerminalCondition;

// Die Nachricht der L0-Trap-Condition, sicher in einer isolierten Speicherregion.
#[link_section = ".vault"]
static TRICKSTER_MESSAGE: &[u8] = b"\x1B[2J\x1B[H\n\r\
    *** TRICKSTER SAGT NEIN, BRO! ***\n\r\
    Zugriffsverletzung erkannt.\n\r\
    Der Vault ist versiegelt.\n\r\
    Komm besser wieder, wenn du gelernt hast.\n\r";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    // Phase 1: Genesis-Condition initialisiert das System.

    // 1. Lease der UART-Ressource für Logging.
    uart::init();
    uart::puts("Corode Genesis-Condition startet...\n");
    
    // 2. PMP-Hardware für Cage-Isolation konfigurieren.
    pmp::init();
    uart::puts("PMP-Grenzen für Cages definiert.\n");
    
    // 3. L0-Trap-Condition (trickster_handler) registrieren.
    asm!("csrw mtvec, {}", in(reg) trickster::trickster_handler as usize);
    uart::puts("L0-Trap-Condition registriert.\n");
    
    // 4. Rent-A-Bunch-System (ZuseAllocator) initialisieren.
    let _zuse = zuse::ZuseAllocator::new();
    uart::puts("Rent-A-Bunch-System (Zuse) bereit.\n");
    
    // 5. Platzhalter für den Training Side Kernel starten.
    let _sidekernel = Sidekernel::new();
    uart::puts("Training Side Kernel (Platzhalter) aktiv.\n");

    // Phase 2: System an die L2-Terminal-Condition übergeben.
    uart::puts("System bereit. Übergebe an L2-Terminal-Condition.\n");
    let terminal = TerminalCondition::new();
    terminal.run(); // Diese Funktion enthält die Hauptschleife.
    
    // Nach der Übergabe an das Terminal sollte dieser Punkt nicht erreicht werden.
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // Ein Panic würde der Corode-Philosophie der garantierten Ausführung widersprechen.
    // In einem reifen System würde dies eine spezielle L0-Fehler-Condition auslösen.
    loop {}
}
