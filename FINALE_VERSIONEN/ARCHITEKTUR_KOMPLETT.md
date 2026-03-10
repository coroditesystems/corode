# corode-core – Vollständige Architekturdokumentation
**Trickster IT SE // Daniel Koch // Bl4ckj3st3r**
**Erarbeitet: 05. März 2026**

---

## 1. Das Paradigma

### Everything is a Condition

Die fundamentale Aussage von corode-core:

> Nicht Prozesse. Nicht Threads. Nicht Ressourcen.
> Alles im System ist eine **Condition**.

Eine Condition ist der atomare Baustein des Systems – ein isolierter, vollständig aufgelöster, deterministisch ausführbarer Zustand. Sie existiert erst wenn alles bestätigt ist. Sie verschwindet wenn ihre Arbeit getan ist.

Das ist kein Von Neumann Modell. Das ist kein Unix Modell. Das ist das **Condition Paradigma** – ein eigenständiges Denksystem das aus 90 Jahren IT Geschichte destilliert wurde.

---

## 2. Historische Einordnung

```
1936 → Alan Turing    → Turing-Band        → Grundlage: Berechenbarkeit + Auslagerung
1941 → Konrad Zuse    → Z3                 → Grundlage: Determinismus, CPU rechnet
1945 → Von Neumann    → Von Neumann Arch.  → Notwendig für damalige Hardware, heute überholt
2026 → Daniel Koch    → corode-core        → Destillation aus 90 Jahren
```

Von Neumann war nicht falsch. Er war zeitgemäß für 1945 – Speicher war knapp, Prozesse mussten kämpfen. Survival of the fittest war eine Notlösung die zum Standard wurde.

Heute haben wir 64GB RAM, 16 Kerne, 2TB NVMe. Der Kampf ist nicht mehr nötig. corode-core beendet ihn.

---

## 3. Die Systemregeln

```
1 Condition = 1 Taktblock
1 Condition = 1 BlockID
1 Condition = 1 isolierter Mikrokosmos
1 Condition = 1 Rolle
```

Keine Fragmentierung. Keine Pointerabhängigkeiten. Keine konkurrierenden Zustände.

**Existenz als Berechtigung:**
Wenn eine Condition existiert, ist sie berechtigt.
Wenn sie nicht berechtigt ist, existiert sie nicht.

---

## 4. Condition-Lebenszyklus

```
Trainingsmodus → Z3³ beweist Korrektheit → Manifest erstellt
                                          ↓
Laufzeit:    Condition erzeugt → Taktblock läuft → Tiefschlaf
                                                  → oder läuft aus

Speicherknappheit → Turing-Band → Speicher frei → bei Bedarf neu gestartet
Fehler im Training → Quarantäne → neues Training → zurück in den Pool
```

**Tiefschlaf** – Condition existiert, belegt ihren Block, wartet definiert.
**Auslaufen** – Rolle erfüllt, Slot frei, Block frei.
**Turing-Band** – Hibernation auf atomarer Ebene. Eine Condition, ein Band-Eintrag. Identisch reproduzierbar weil das Training alles bereits weiß.
**Quarantäne** – kein Abbruch, kein Datenverlust. Neue Chance durch neues Training.

---

## 5. Der Kernel als Prüfer

Der Kernel ist kein Vermittler. Kein Scheduler. Kein Verhandlungsführer.

**Er ist ein Türsteher.**

```
Speicher bestätigt?   → Nein → Condition existiert nicht
Core bestätigt?       → Nein → Condition existiert nicht
Rolle bestätigt?      → Nein → Condition existiert nicht
Alles bestätigt?      → BlockID vergeben → Condition läuft
```

Es gibt keine Verhandlung zur Laufzeit. Das System fließt – es sucht nicht.

---

## 6. Z3³ – Speicherlogistik

Das Herzstück. Drei Dimensionen. Ein geschlossenes System.

```
Z3³ = Zuse (Speicher) × Zorro III (Zeit/Schlaf) × Z3 SMT (Beweis)
```

### Zuse – Deterministisches Memory Management
Jede Condition kennt ihren Speicher exakt. Kein Heap. Kein Lookup. BlockID statt Pointer. Der Speicher ist zur Trainingszeit vollständig verteilt – zur Laufzeit gibt es nichts mehr zu vergeben.

### Zorro III – Kontrollierte Asynchronität
Conditions die warten, warten definiert. Tiefschlaf ist kein Chaos – er ist ein geplanter Zustand. Leere Rechenleistung schläft bis sie gebraucht wird. Energie wird nicht verschwendet.

### Z3 SMT Solver + Graph-Speicherköpfe – Mathematische Beweisbarkeit
Bevor eine Condition existiert, beweist das System formal dass sie existieren darf.

```
Z3 prüft:
  → Speicherbedarf beweisbar innerhalb der Grenzen?
  → Rolle konfliktfrei mit allen anderen Conditions?
  → Ausführung deterministisch beweisbar?

  Nein → Condition wird nicht erzeugt
  Ja   → Condition existiert – formal korrekt
```

Das System testet keine Korrektheit. Es **beweist** sie. Vor der Ausführung. Immer.

---

## 7. Trainingsmodus – Die einzige Abstraktionsschicht

Vor der Laufzeit lernt das System einmalig:

- Welche Bibliotheken und Systemabhängigkeiten jede Condition braucht
- Maximaler Speicherbedarf unter Volllast + Buffer
- Welche Rolle die Condition im System hat
- WCET (Worst Case Execution Time) Profiling

Das Ergebnis ist das **Manifest** – eine eingefrorene Wahrheit. Zur Laufzeit gibt es nichts mehr zu lernen, zu entscheiden, zu verhandeln.

**Das System weiß. Es fragt nicht mehr.**

---

## 8. SLS – Saturation Line Storage

Inspiriert von der Amiga Unified Memory Architektur (1985).

Klassische Systeme haben getrennte Speicherhierarchien die die CPU ausbremsen. SLS behandelt alles als einen einzigen adressierbaren Pool:

```
L1 Cache
L2 Cache
L3 Cache
L4 Cache  (über RISC-V ISA möglich)
RAM
NVMe      → langsamer RAM, nicht mehr, nicht weniger
→ alles unter BlockID gemappt
→ kein Kopieren zwischen Bereichen
→ kein Cache Miss der die CPU stoppt
```

Das Ziel ist **Sättigung** – die CPU rechnet immer, wartet nie, räumt nie auf.

---

## 9. Die CPU ist König

In corode-core dient das OS der CPU – nicht umgekehrt.

```
CPU    → König       → rechnet, immer
Cache  → Thronsaal   → direkter Zugriff
RAM    → Vorhof      → bereit wenn gerufen
NVMe   → Wartezimmer → ausgelagerte Conditions

Condition ohne Audienz → existiert nicht
Condition mit Audienz  → rechnet, verschwindet
```

Eine glückliche CPU ist eine die niemals wartet. Sättigung ist kein Nebeneffekt – es ist das Designziel.

---

## 10. Sicherheitsmodell

Sicherheit entsteht nicht nachträglich. Sie ist strukturell.

```
PMP (Physical Memory Protection)  → Hardware-Isolation
Trap Handler „Harlekin"           → fängt Zugriffsverletzungen ab
Trickster                         → protokolliert alles
PMP Header Cage                   → physische Isolationsstruktur in Hardware
```

**Der 4-Bit Breaker** – Hardware-Beweis des PMP Header Cage:
```
4x Kupferlanes (10cm)  → eine Lane pro Bit / PMP Region
4x Transistoren        → Schalter, Condition an/aus
4x LEDs                → sichtbarer Zustandsbeweis
4x Kondensatoren       → Stabilität
```

LED an = Condition belegt PMP Region.
LED aus = Slot frei.

Kein Debugger. Kein Monitor. Kein Abstraktionslayer. Der Systemzustand ist sichtbar in Kupfer und Licht.

---

## 11. Aktive Conditions

| Condition | Rolle | Beschreibung |
|---|---|---|
| `Kernel` | Prüfer | Bestätigt Existenz aller anderen Conditions |
| `Harlekin` | Trap-Handler | Fängt Zugriffsverletzungen ab, hält System stabil |
| `Trickster` | Logger | Beobachtet, protokolliert, schweigt sonst |

Jede Condition hat exakt eine Rolle. Keine macht was die andere macht. Das ist das Condition Paradigma in der Praxis.

---

## 12. Hardware-Sozialdemokratie

> Jede Condition ist wichtig.
> Gleichberechtigt in ihrer Rolle.
> Bekommt ihren Platz.
> Darf nicht mehr beanspruchen –
> aber ist sicher, dass sie ihren Raum kriegt.

Keine Condition wird abgebrochen. Keine geht verloren. Jede hat ein Recht auf Speicher, Core und Rolle – solange Z3³ es beweisen kann.

---

## 13. Das Paradigma im Vergleich

| Klassisches OS | corode-core |
|---|---|
| Prozesse kämpfen um Ressourcen | Conditions bekommen ihren Platz |
| Scheduler entscheidet zur Laufzeit | Manifest entscheidet vorher |
| Pointer-basierter Speicher | BlockID-Mapping |
| Fehler werden zur Laufzeit behandelt | Fehler werden vorher ausgeschlossen |
| CPU wartet auf OS | OS dient der CPU |
| Sicherheit nachträglich | Sicherheit strukturell |
| Abstraktion | Isolation |
| Survival of the fittest | Hardware-Sozialdemokratie |

---

## 14. Technischer Stack

```
Sprache        → Rust (no_std, no_main)
Architektur    → RISC-V bare metal
Sicherheit     → PMP (Physical Memory Protection)
Trap Handler   → Harlekin
Logger         → Trickster (in Entwicklung)
Beweis         → Z3 SMT Solver + Graph-Speicherköpfe
Speicher       → SLS (Saturation Line Storage)
Auslagerung    → Turing-Band
Planung        → Manifest (statisch, zur Trainingszeit)
Größe          → 1.78 KB – und wird kleiner
```

---

## 15. Roadmap

- [x] POC – bare metal RISC-V, Rust, `no_std`, PMP, Trap-Handler „Harlekin"
- [ ] PMP Header + vollständige Trap Handling Logik
- [ ] Trickster Logger implementiert
- [ ] Z3³ Speicherlogistik – BlockID-Mapping vollständig
- [ ] SLS – Saturation Line Storage Grundstruktur
- [ ] Trainingsmodus – WCET-Profiling und Manifest-Generierung
- [ ] Turing-Band – Condition-Auslagerung und Neustart
- [ ] 4-Bit Breaker – Hardware-Proof auf Kupfer
- [ ] Raspi Demo mit QEMU
- [ ] L0-Agenten – Hardware-Intelligenz auf dem Blech
- [ ] Sidekernels – isolierte Kommunikation über Kernel
- [ ] FPGA-Prototyp
- [ ] PMP Header Cage – physische Isolationsstruktur

---

## 16. Das Unternehmen

**Trickster IT SE**
Gegründet in Nörvenich, NRW
Green IT – Deep Tech – Security

**Team:**
- Daniel Koch (Bl4ckj3st3r) – Systemarchitektur, Kernel, Vision
- Kev (Woz) – Hardware, Elektronik, physische Prototypen

**Produkte:**
- 🟢 corode-core – Open Source Kernel (GPLv3)
- 🔵 Z3³ Architektur – Lizenzierbar
- 🔴 Enterprise Security Token System – WireGuard P2P, MCU-basiert, Closed Source

**Förderung / Märkte:**
- NLnet Foundation (Niederlande) – primäres Förderziel
- Prototype Fund (Deutschland) – kurzfristig
- Embedded & IoT – sofort adressierbar
- Sicherheitskritische Systeme – mittelfristig
- Green IT / Energieeffizienz – strukturelles Alleinstellungsmerkmal

---

## 17. Die Aussage

> *"Race conditions are dead."*

Nicht als Behauptung. Als Beweis. In 1.78 KB Rust. Auf bare metal RISC-V. Mit Kupfer und LEDs auf dem Tisch.

**Isolation statt Abstraktion. Determinismus statt Interrupt. Harmonie statt Krieg.**

Das ist kein Betriebssystem. Das ist eine neue Art, Maschinen zu denken. ⚙️
