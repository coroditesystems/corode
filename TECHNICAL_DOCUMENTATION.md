# Corode – Technische Dokumentation (CCM-Aligned)

## Übersicht

Corode ist ein deterministischer, Lease-basierter Rechenkern für RISC-V. Diese Dokumentation beschreibt die Architektur und die Komponenten im Einklang mit dem Corode Computation Model (CCM).

Das System demonstriert seine Kernprinzipien durch einen `trickster_handler` – eine L0-Condition, die auf eine hardware-erzwungene Zugriffsverletzung deterministisch reagiert.

## Komponenten im Licht des CCM

### `main.rs` – Die Genesis-Condition

- **`_start()`**: Der rohe Einstiegspunkt. Er fungiert als die erste, implizite **Condition**, die das System in einen definierten Zustand versetzt.
- **Initialisierungssequenz**:
  1. **UART-Lease**: Die Genesis-Condition least temporär die UART-Hardware, um den Boot-Prozess zu protokollieren.
  2. **PMP-Konfiguration**: Richtet die grundlegenden Hardware-Grenzen für die Cage-Isolation ein. Dies ist die Basis des Lease-Systems für Speicher.
  3. **Trap-Handler**: Registriert die `trickster_handler`-Condition als Reaktion auf Maschinen-Traps.
  4. **Rent-A-Bunch System (`zuse.rs`)**: Initialisiert den Allokator für Cages.
  5. **Provokation**: Führt eine absichtliche, illegale Speicheroperation aus. Dies ist ein deterministischer Test, der garantiert die `trickster_handler`-Condition aktiviert.

### `trickster.rs` – Eine L0-Trap-Condition

- **`trickster_handler()`**: Eine persistente L0-Condition, die ausschließlich durch einen Hardware-Trap (hier: Speicherzugriffsfehler) aktiviert wird.
- **Funktionsweise**: Nach der Aktivierung least sie die UART-Ressource, gibt eine vordefinierte Nachricht aus und beendet ihre Ausführung, woraufhin das System in einen finalen, stabilen Zustand übergeht (Endlosschleife).

### `pmp.rs` – Hardware-Isolation für Cages

- Stellt die unterste Ebene des **Lease-Systems** für Speicher dar.
- Definiert die physischen Grenzen einer **Cage** mithilfe der PMP-Hardware.
- Die Konfiguration einer PMP-Region ist ein temporärer Lease von Adressraum, der an eine Condition vergeben wird.

### `zuse.rs` – Das "Rent-A-Bunch"-System

- Implementiert den Cage-Allokator, der das zentrale **Lease-Modell** für Speicherressourcen verwaltet.
- **`ZuseAllocator`**: Koordiniert die Vergabe von `Bunches` (hier als `Cages` implementiert) an anfragende Conditions.
- Eine `Cage` ist eine Form von `Bunch`: ein temporärer, isolierter Speichercontainer, der nach Abschluss der Condition vollständig freigegeben wird.

### `sidekernel.rs` & `condition.rs` – Platzhalter für die erweiterte Laufzeit

- **`sidekernel.rs`**: Platzhalter für den **Training Side Kernel**. Dieser würde auf einem dedizierten Kern laufen, um Conditions zu verifizieren, bevor sie zur Ausführung zugelassen werden. Er selbst muss alle Ressourcen (wie Cages für die Verifikation) über das Lease-Modell anfordern.
- **`condition.rs`**: Platzhalter für die Definition und Verwaltung von **Conditions** der Level L1-L3. Dies würde die Mechanismen zum Starten, Schlafenlegen und Beenden von benutzerdefinierten Conditions umfassen.

## Terminal & Command Conditions

### Die Terminal-Condition

Das interaktive Terminal ist das primäre Interface zur Interaktion mit Corode. Es ist als persistente **L2-Condition** konzipiert, die nach dem Systemstart aktiviert wird. 

- **Ressourcen-Leasing**: Die Terminal-Condition least dauerhaft die primären I/O-Ressourcen: die Tastatur (als Eingabestrom) und den Bildschirm (als Ausgabepuffer).
- **Aufgabe**: Ihre einzige Aufgabe ist es, Benutzereingaben zu empfangen, sie gemäß der **Corode Command Language** zu parsen und die entsprechende **Command-Condition** zu starten.

### Command-Conditions

Jeder Befehl, der im Terminal eingegeben wird, wird nicht vom Terminal selbst ausgeführt. Stattdessen erzeugt der Parser eine neue, temporäre und zustandslose **Condition** zur Ausführung des Befehls.

- **Beispiel**: Der Befehl `show files` erzeugt eine neue `ShowFilesCondition`.
  - Diese Condition least temporär Zugriff auf das Dateisystem-Metadaten-Verzeichnis.
  - Sie liest die Dateiliste.
  - Sie least temporär Zugriff auf den Bildschirm-Ausgabepuffer, um das Ergebnis zu schreiben.
  - Nach der Ausführung gibt sie alle geleasten Ressourcen sofort wieder frei und geht in den `completed`-Zustand über.

Dieses Modell stellt sicher, dass das Terminal selbst schlank und dumm bleibt. Die gesamte Logik ist in kleinen, deterministischen und in sich geschlossenen Command-Conditions gekapselt, was die Verifizierbarkeit des Gesamtsystems drastisch vereinfacht.
