# Corode - Technische Dokumentation

## 1. Vision: Ein beweisbar korrektes Betriebssystem

Corode ist der Versuch, ein Betriebssystem zu schaffen, dessen Verhalten nicht nur getestet, sondern formal bewiesen wird. Anstatt auf komplexe, zur Laufzeit getroffene Entscheidungen (wie Scheduling oder dynamische Speicherverwaltung) zu setzen, wird das gesamte Systemverhalten zur Compile-Zeit in einem **Manifest** definiert und mit einem SMT-Solver (wie Z3) auf Korrektheit und Stabilität überprüft.

## 2. Das Kernkonzept: Die Condition

Alles im System ist eine **Condition**: ein isolierter, vollständig aufgelöster, deterministisch ausführbarer Zustand. Eine Condition ist der atomare Baustein des Systems.

- **Isolation:** Jede Condition läuft in einem eigenen, durch Hardware (RISC-V PMP) geschützten Speicherbereich.
- **Determinismus:** Der Code einer Condition ist einfach und in seiner Ausführungszeit vorhersagbar. Er hat keinen Zugriff auf blockierende Operationen oder komplexe Schleifen.
- **Formale Verifikation:** Eine Condition "existiert" nur, wenn ihre Anforderungen (Speicher, Rechenzeit, Zugriff auf andere Conditions) die im Manifest festgelegten Systemregeln nicht verletzen.

## 3. Genesis-Implementierung: Der erste lauffähige Beweis

Das aktuelle System (`main.rs`) ist die "Genesis"-Implementierung. Es beweist die grundlegende Machbarkeit auf Bare-Metal-RISC-V.

### Komponenten

- **`main.rs` (`kmain`)**: Der Einstiegspunkt nach dem Bootloader. Initialisiert die UART, richtet eine grundlegende PMP-Region ein und startet die deterministische Hauptschleife.
- **`state.rs` (`CoreState`)**: Hält den globalen Zustand des Cores, im Wesentlichen die `CondMask`, eine Bitmaske, die anzeigt, welche Conditions gerade aktiv sind.
- **`cond.rs` (`CondMask`, `id`)**: Definiert die Bitmaske für Conditions und weist jeder benannten Condition eine eindeutige ID (eine Bit-Position) zu.
- **`block.rs` (`Block`, `run_blocks`)**:
    - Ein `Block` ist die praktische Implementierung einer Zustandsänderung. Er enthält die auszuführende Funktion (`run`) und die Bedingungen (`require_all`, `require_none`), unter denen er laufen darf.
    - Die Funktion `run_blocks` ist das Herz des Systems. In einer Endlosschleife iteriert sie über alle statisch definierten `BLOCKS` und führt diejenigen aus, deren Bedingungen durch die aktuelle `CondMask` im `CoreState` erfüllt sind. Dies ist der "Scheduler" – vollständig deterministisch und ohne Priorisierung.
- **`pmp.rs`**: Funktionen zur Konfiguration der Physical Memory Protection (PMP) von RISC-V. Dies ist die Hardware-Grundlage für die Isolation von Conditions.
- **`uart.rs`**: Einfache, Polling-basierte UART-Treiber für die serielle Ausgabe.
- **`trap.rs`**: Eine rudimentäre Trap-Handler-Struktur, die aktuell noch keine Funktion hat.

### Systemablauf (Genesis)

1.  **`kmain`**:
    - Initialisiert UART.
    - Richtet eine globale PMP-Region ein, die den gesamten Speicher für den Kernel freigibt.
    - Erstellt einen neuen `CoreState`.
    - Setzt die initiale Condition `PMP_OK`.
2.  **`loop` in `kmain`**:
    - Ruft `run_blocks` auf.
    - `run_blocks` prüft alle `BLOCKS` in der statischen `BLOCKS`-Tabelle.
    - Der `boot_done`-Block wird ausgeführt, da seine Bedingung (`require_none: BOOT_DONE`) erfüllt ist. Er setzt die `BOOT_DONE`-Condition.
    - In der nächsten Iteration wird der `boot_done`-Block nicht mehr ausgeführt.
    - Das System verbleibt in diesem stabilen, leeren Zustand und wartet auf externe Interrupts (die noch nicht implementiert sind), um neue Conditions zu setzen.
