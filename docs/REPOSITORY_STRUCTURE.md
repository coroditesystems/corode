# Repository-Struktur: Eine Landkarte von corode-core

Dieses Dokument beschreibt die Organisation des `corode-core` Repositories. Die Struktur ist darauf ausgelegt, eine klare Trennung zwischen Code, Dokumentation und Werkzeugen zu gewährleisten. Sie ist logisch, skalierbar und folgt den Best Practices für Betriebssystem- und Embedded-Projekte.

---

## Die Verzeichnisstruktur

```
/
├── .github/            # CI/CD und GitHub-Automatisierungen
├── docs/               # Die Bibliothek: Designdokumente, Manifeste, Spezifikationen
├── scripts/            # Werkzeuge zum Bauen, Testen und Ausführen des Kernels
├── src/                # Der Kern-Quellcode von corode-core in Rust
├── sidekernels/        # Gekapselte Projekte für untergeordnete Sidekernels
│   ├── linux/
│   └── ai/
├── .gitignore          # Definiert, welche Dateien von Git ignoriert werden sollen
├── Cargo.toml          # Rust-Projektmanifest (Abhängigkeiten, Build-Konfiguration)
└── README.md           # Das Aushängeschild des Projekts: Eine schnelle Übersicht
```

---

## Detaillierte Beschreibung

### `/` (Root-Verzeichnis)

*   `README.md`: Die erste Anlaufstelle. Sie bietet eine prägnante Zusammenfassung der Projektphilosophie, des aktuellen Status und der Vision. Sie dient als Manifest und Aushängeschild.
*   `Cargo.toml`: Das Herzstück jedes Rust-Projekts. Definiert das Projekt, seine Abhängigkeiten und die Build-Konfigurationen für den `corode-core` Kernel.
*   `.gitignore`: Eine Konfigurationsdatei für Git, die sicherstellt, dass Build-Artefakte (z. B. der `target`-Ordner), temporäre Dateien und IDE-spezifische Konfigurationen nicht im Repository landen. Das hält die Versionskontrolle sauber und fokussiert.

### `src/`

Das Herz des Projekts. Dieses Verzeichnis enthält den gesamten Quellcode für den `corode-core` Kernel selbst.

*   `main.rs`: Der Einstiegspunkt unseres Bare-Metal-Kernels. Hier beginnt die Ausführung, hier wird die Hardware initialisiert und die PMP-Firewall konfiguriert.

### `docs/`

Die Bibliothek. Hier wohnt das Wissen und die Vision des Projekts. Dieses Verzeichnis trennt die *Idee* vom *Code*.

*   `PMP_VEKTOR_MAP.md`: Die "Verfassung" des Systems. Definiert die feste, in Hardware erzwungene Speicheraufteilung und die fundamentalen Sicherheitsregeln.
*   `EASTEREGGS.md`: Die kulturelle Seele des Projekts.
*   `REPOSITORY_STRUCTURE.md`: Dieses Dokument.

### `sidekernels/`

Die Vorbereitung für Phase Zwei unserer Strategie: die Übernahme nützlicher, aber unzuverlässiger Systeme. Jeder Sidekernel (z. B. ein gekapselter Linux-Kernel für Treiber oder eine AI-Engine) wird als eigenständiges, isoliertes Projekt in diesem Verzeichnis entwickelt. Sie sind Diener, keine Herren.

### `scripts/`

Das Arsenal. Dieses Verzeichnis sammelt alle Hilfsskripte, die zum Bauen, Ausführen, Testen und Debuggen des Kernels benötigt werden (z.B. QEMU-Startskripte, GDB-Integrationen).

### `.github/`

Das Automatisierungszentrum. Enthält Workflows für Continuous Integration (CI), die bei jedem Push automatisch sicherstellen, dass der Code kompiliert und Tests bestanden werden.
