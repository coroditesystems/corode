//! Die persistente L2-Terminal-Condition.
//!
//! Diese Condition least die I/O-Ressourcen (Tastatur, Bildschirm) und ist 
//! dafür verantwortlich, Benutzereingaben zu empfangen und in dedizierte
//! Command-Conditions zur Ausführung zu überführen.

pub struct TerminalCondition;

impl TerminalCondition {
    /// Initialisiert die Terminal-Condition und least die I/O-Ressourcen.
    pub fn new() -> Self {
        // Hier würde der Lease für Tastatur und Bildschirm stattfinden.
        Self
    }

    /// Startet die Hauptschleife der Terminal-Condition.
    pub fn run(&self) {
        loop {
            // 1. Auf Eingabe warten (z.B. von der Tastatur-Ressource).
            // 2. Eingabe parsen (z.B. mit einer Funktion aus `commands.rs`).
            // 3. Entsprechende Command-Condition erzeugen und zur Ausführung anfordern.
        }
    }
}
