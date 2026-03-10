# corode-core

**Motto:** Isolation statt Abstraktion. Determinismus statt Chaos.

`corode-core` ist ein von Grund auf neu entwickelter, in Rust geschriebener Bare-Metal-Kernel für die RISC-V-Architektur. Unser Ziel ist es, ein beweisbar sicheres und deterministisches System zu schaffen, indem wir die Hardware-Sicherheitsfeatures der CPU (Physical Memory Protection) als primäres Ordnungsprinzip nutzen.

## Kernprinzipien

*   **Hardware-Sozialdemokratie:** Der physische Speicher ist in 16 unveränderliche, hardware-gesicherte "Vektoren" aufgeteilt. Jeder System-Komponente wird genau der Speicher zugewiesen, den sie zum Überleben braucht – nicht mehr und nicht weniger. Die Gesetze sind in Silizium gegossen.
*   **Isolation statt Abstraktion:** Wir vermeiden komplexe Software-Abstraktionen. Stattdessen erzwingen wir eine strikte Isolation direkt auf der Hardware-Ebene mittels RISC-V PMP.
*   **Conditions statt Prozesse:** Unser Ausführungsmodell basiert auf `Conditions` – zustandslose, überprüfbare Code-Einheiten, deren Korrektheit vor der Ausführung durch einen SMT-Solver (wie Z3) bewiesen werden kann.

## Aktueller Status: Meilenstein 1 ✅

Das Fundament ist fertig, stabil und bewiesen.

1.  **Stabiler Kernel-Prototyp:** Der `corode-core` Kernel (`src/main.rs`) bootet, konfiguriert die Hardware-Firewall (PMP) und übernimmt die vollständige Systemkontrolle.
2.  **Hardware-Firewall Aktiv:** Alle 16 Speicher-Vektoren sind gemäß unserer Verfassung (`docs/PMP_VEKTOR_MAP.md`) konfiguriert und versiegelt.
3.  **Sicherheit Bewiesen:** Ein integrierter Selbsttest beweist zur Boot-Zeit, dass die PMP-Schilde halten und das System selbst Angriffe von innen abwehrt.

## Nächste Schritte

Die nächsten Schritte sind in unserer Roadmap klar definiert:

1.  **Sidekernel-Integration:** Implementierung der Kommunikationsprotokolle für die gekapselten Sidekernels (AI, Linux-Treiber, Unix-Userland).
2.  **Z3³-Speicherlogistik:** Vollständige Implementierung des BlockID-basierten Speichermodells.

## Dokumentation

*   **[Repository-Struktur](docs/REPOSITORY_STRUCTURE.md):** Eine Landkarte des Projekts.
*   **[PMP Vektor-Karte](docs/PMP_VEKTOR_MAP.md):** Die "Verfassung" unseres Systems, die die Speicheraufteilung definiert.
*   **[Easter Eggs](docs.EASTEREGGS.md):** Die kulturelle Seele des Projekts.

---

**The Pink Hacker Boy**  
Eifel, 2026  
💖🎒🤡🥚😎
