//! trap.rs - Eine rudimentäre Trap-Handler-Struktur.

use crate::uart::uart_puts;

/// Diese Funktion wird theoretisch bei einem Trap aufgerufen. Momentan ist sie
/// noch nicht im Trap-Vektor registriert und dient als Platzhalter.
pub fn unimplemented_trap() {
    // Easter Egg: "It's a trap!" - Admiral Ackbar
    // Perfekt für eine Funktion, die eigentlich Fehler abfangen sollte.
    uart_puts("It's a trap!\n");
}

// Zukünftige Erweiterungen könnten hier folgen:
// - Echte Trap-Handler für verschiedene Ursachen (Exceptions, Interrupts)
// - Eine Funktion zur Initialisierung der Trap-Vektor-Tabelle (mtvec)
