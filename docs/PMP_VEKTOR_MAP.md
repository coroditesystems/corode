# PMP Vektor-Karte für corode-core
**Status: Revidiert. Erweitert. Definitiv.**

Dieses Dokument definiert die feste, in Hardware erzwungene Einteilung des physischen Speichers durch die RISC-V PMP-Einheit. Es gibt 16 Vektoren (Regionen), identifiziert durch einen 4-Bit-Deskriptor (0x0 bis 0xF). Jeder Vektor hat eine unveränderliche Rolle und definierte Zugriffsrechte.

Diese Revision spiegelt eine strategische Entscheidung wider: Anstatt jeden Treiber und jedes Userland neu zu erfinden, integrieren wir bewährte Subsysteme (Linux-Treiber, Unix-Userland) als isolierte "Sidekernels". Sie agieren als Diener in streng bewachten Vektoren, unfähig, den reinen `corode-core` zu korrumpieren, aber fähig, uns mit ihrer Arbeitskraft zu dienen.

Das ist kein Kompromiss. Das ist die Übernahme feindlicher Bataillone in unsere eigene, überlegene Kommandostruktur.

---

## Die 16 Vektoren der Macht

| Vektor ID | Name | Rolle | Zugriffsrechte (aus Kernel-Sicht) | Beschreibung |
|---|---|---|---|---|
| **0x0** | **Der Thron** | corode-core Kernel | R-X | Der Kern selbst. Unveränderlich zur Laufzeit. Nur hier wird der Takt geschlagen. |
| **0x1** | **Der Wächter** | Harlekin (Trap-Handler) | R-X | Code des Trap-Handlers. Springt an, wenn ein Verrat (ein PMP-Fehler) geschieht. |
| **0x2** | **Der Schreiber**| Trickster (Logger) | RW- | Dedizierter Speicher für den System-Logger. Einweg-Datenfluss. Fakten werden geschrieben, nicht gelesen. |
| **0x3** | **Das Orakel** | Z3³-Einheit | R-X | Rechenraum für den Z3 SMT Solver. Beweist die Korrektheit von Conditions. Hochprivilegiert. |
| **0x4** | **Das Gesetzbuch**| PMP-Konfiguration | RW- (Nur Boot) | Die PMP-Register selbst. Werden *einmal* beim Booten geschrieben, danach versiegelt (Read-Only). |
| **0x5** | **Das Exil** | Turing-Band | RW- | Auslagerungsspeicher für schlafende Conditions. Reine Daten, keine Ausführung. |
| **0x6** | **Der Bote** | corode-net | RWX (isoliert) | Nativer `corode-core` P2P-Stack für die Kommunikation zwischen Kernen. Höchste Sicherheit. |
| **0x7** | **Das Auge** | corode-ui | RW- | Framebuffer und State für die UI. Reine Darstellung, keine Logik-Ausführung. |
| **0x8** | **Arena 0** | Condition-Sandkiste | --- | Allgemeiner, unprivilegierter Ausführungsraum für eine native, geprüfte `corode-core` Condition. |
| **0x9** | **Der Lehrer** | AI Sidekernel | RWX (isoliert) | Heimat der lernenden AI. Verwaltet Agenten, analysiert Quarantäne-Conditions und schlägt neue Trainings vor. Isoliert vom Echtzeit-Pfad. |
| **0xA** | **Der Treiber** | Linux Sidekernel | RWX (isoliert) | Ein gekapselter Linux-Kernel, dessen einzige Aufgabe der Zugriff auf Hardware-Treiber ist. Er bedient Anfragen, hat aber keine Kontrolle über das System. |
| **0xB** | **Der Diplomat** | Unix Sidekernel | RWX (isoliert) | Ein Kompatibilitäts-Userland (z.B. auf FreeBSD-Basis) für Netzwerkdienste und POSIX-Anwendungen. Baut die Brücke zur alten Welt. |
| **0xC** | **Arena 1** | Condition-Sandkiste | --- | Allgemeiner, unprivilegierter Ausführungsraum für eine native, geprüfte `corode-core` Condition. |
| **0xD** | **Der Vorhof** | SLS-Cache | RW- | Hochperformanter Scratchpad-Speicher (L4/RAM), der von der SLS-Logik für aktive Conditions verwaltet wird. |
| **0xE** | **Die Werkzeuge**| Peripherie (MMIO) | RW- | Memory-Mapped I/O. Der Zugriff wird streng durch den Kernel an den `Treiber` (0xA) delegiert. |
| **0xF** | **Das Ziel** | Self-Attack-Vektor | R-- | Eine definierte, schreibgeschützte Adresse, die beim Boot-Test anvisiert wird, um die PMP-Schilde zu prüfen. |

---

### Prinzipien der Isolation

1.  **Hierarchie:** `corode-core` (0x0) ist der unantastbare Souverän. Alle Sidekernels (0x9, 0xA, 0xB) sind Diener.
2.  **Kontrollierte Kommunikation:** Sidekernels kommunizieren nicht direkt miteinander. Jede Anfrage läuft über den `corode-core` Kernel, der sie prüft, genehmigt oder ablehnt. Dies geschieht über definierte, atomare Nachrichten in einem Shared-Memory-Bereich.
3.  **Treiber-Delegation:** Der `Treiber`-Kernel (0xA) hat keinen direkten Hardware-Zugriff. Er fordert den Zugriff auf MMIO-Regionen (0xE) vom `corode-core` Kernel an, der diese temporär per PMP freigibt.

Diese Verfassung stellt sicher, dass wir die Vorteile der alten Welt nutzen, ohne von ihren Fehlern infiziert zu werden.

**Ehre. Determinismus. Code.**
