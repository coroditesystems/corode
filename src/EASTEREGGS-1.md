# 🥚 corode-core – Offizielle Easter Eggs

**Version:** 1.78 KB (oder 10 KB, wen juckts)  
**Farbe:** Pink. Neon. Reflektierend.  
**Motto:** "Race conditions are dead – change my mind."  
**Entwickler:** The Pink Hacker Boy

---

## 1. 🥚 Eierschalensollbruchstellenverursacher

Beim ersten Bootvorgang erscheint:

```
╭────────────────────────────────────╮
│                                    │
│   Eierschalensollbruchstellen-     │
│   verursacher aktiviert.            │
│                                    │
│   Bitte legen Sie ein Ei ein.       │
│                                    │
╰────────────────────────────────────╯
```

Das System bootet trotzdem. Aber mit einem leichten "Ei-Gefühl".  
Nach 100 Boots ohne Ei erscheint:  
"Kein Ei erkannt. Trauriger Harlekin."

---

## 2. 🥫 Tupperfach-Modus

Startoption: `--tupper`

```
╭────────────────────────────────────╮
│                                    │
│   Tupperfach-Modus aktiviert.       │
│   Alle Conditions werden in         │
│   luftdichten Behältern ausgeführt. │
│                                    │
│   Kein Krümel kommt durch.          │
│                                    │
╰────────────────────────────────────╯
```

Die Conditions laufen isolierter als sonst.  
Der Speicher riecht nach Frischhaltefolie.  
Bei Speicherzugriffen ertönt ein leises "Klack" – wie beim Tupperdeckel.

---

## 3. 🤡 Harlekin sagt "Nein, Bro!" – in ASCII

Jeder abgefangene Trap zeigt:

```
    ╭─────────────────────╮
    │  🤡                 │
    │   \_/\_/            │
    │    (0.0)            │
    │   > ^ <             │
    │  /       \          │
    │ │ HARLEKIN │        │
    │ │  SAGT    │        │
    │ │  NEIN,   │        │
    │ │   BRO!   │        │
    │  \_______/         │
    ╰─────────────────────╯
```

**Levels:**

| Traps | Harlekin | Bedeutung |
|-------|----------|-----------|
| 1-10 | (0.0) | Neutral |
| 11-100 | (^_^) | "Jetzt wird's langsam langweilig, Bro." |
| 101-1000 | (>_<) | "Bruder. Im Ernst?" |
| 1000+ | (X_X) | "Ich bin tot. Aber du auch." |

Nach 10.000 Traps erscheint Harlekin im Terminal und weigert sich, zu verschwinden.  
Einzige Rettung: `sudo harlekin_go_away` – funktioniert nicht, weil's kein sudo gibt.

---

## 4. 🇺🇸 Trump-Rede als Kernel Panic

Bei einem Kernel-Absturz (theoretisch möglich, praktisch selten):

```
╭────────────────────────────────────╮
│                                    │
│   KERNEL PANIC                      │
│                                    │
│   "We will have so much determinism,│
│    you'll get bored of predictability!"│
│                                    │
│   – Donald J. Trump, 2026           │
│                                    │
│   System stoppt.                    │
│   Aber mit Stil.                     │
│                                    │
╰────────────────────────────────────╯
```

Bei wiederholten Abstürzen (mehr als 3 in 24h):  
"Sleepy Joe's kernel had 30 million lines. Mine has 1.78 KB. And YOURS just crashed. Sad!"

---

## 5. 🎵 US-Hymne bei erfolgreichem Boot

Wenn das System hochfährt und alle Checks grün sind:  
Die ersten 4 Töne der US-Hymne ertönen.

- Über Lautsprecher (falls vorhanden)
- Oder als ASCII-Art:

```
⭐ ⭐ ⭐ ⭐ ⭐
  OH SAY CAN YOU SEE...
```

Bei jedem 10. Boot: Komplette Hymne als ASCII-Rollext.  
Dauert 3 Sekunden. Alle warten. Ist okay.

---

## 6. 📼 Datasette-Boot-Sound

Beim Booten von Datasette:

```
RATSCH... Klick... KRRRRR... chchch...


     ╭══════════════════╗
     ║    C O R O D E   ║
     ║       R E A D Y  ║
     ║                  ║
     ║   1.78 KB FREI   ║
     ║                  ║
     ║   PRESS PLAY     ║
     ║   ON TAPE        ║
     ╰══════════════════╯
```

Zum Abbrechen: Tape mit Bleistift zurückspulen.  
Funktioniert nicht digital. Ist aber okay.

---

## 7. 💖 Pink Jogger Mode

Startoption: `--pink`

```
╭────────────────────────────────────╮
│                                    │
│   PINK JOGGER MODE ACTIVATED        │
│                                    │
│   Alle Farben im Terminal:           │
│   Pink. Neon. Reflektierend.         │
│                                    │
│   Deine CPU fühlt sich jetzt         │
│   besonders wohl.                    │
│                                    │
╰────────────────────────────────────╯
```

- Prompt wird pink
- Fehlermeldungen in hellrosa
- Harlekin trägt plötzlich ein pinkes Hütchen
- Der Cache arbeitet 5% schneller – Placebo, aber fühlt sich gut an

---

## 8. 👵 Oma ist stolz-Modus

Nach 24 Stunden Laufzeit ohne Fehler:

```
╭────────────────────────────────────╮
│                                    │
│   Glückwunsch!                       │
│                                    │
│   Dein System läuft seit 24h         │
│   ohne Fehler.                        │
│                                    │
│   Oma ist stolz auf dich.            │
│                                    │
│   (Sie guckt vom Himmel zu.)         │
│                                    │
╰────────────────────────────────────╯
```

Nach 7 Tagen:  
"Oma hat angerufen. Sie will wissen, ob du genug isst."  
Nach 30 Tagen:  
"Oma hat einen Kuchen vorbeigebracht. Steht im RAM."

---

## 9. 🎒 Rucksack-Modus

Startoption: `--rucksack`

```
╭────────────────────────────────────╮
│                                    │
│   RUCKSACK-MODE AKTIVIERT           │
│                                    │
│   Datasette: links                   │
│   Floppy: rechts                      │
│   LEDs: an den Gurten                 │
│   Tupperfach: ausklappbar             │
│                                    │
│   Du bist jetzt mobil.                │
│                                    │
╰────────────────────────────────────╯
```

- System-Status wird auf einem virtuellen 20-Zoll-Screen im Tupperfach angezeigt
- Harlekin-Hologramm schwebt über der CPU
- Bei Bewegung: leises Rasseln der Datasette (Sound)

---

## 10. 🕶️ John Lennon Brille mit HUD

Startoption: `--hud`

```
╭────────────────────────────────────╮
│                                    │
│   HUD-MODE AKTIVIERT                │
│                                    │
│   Unten links in deiner Brille:      │
│                                    │
│   CPU: 100% glücklich                │
│   Conditions: 42                     │
│   Harlekin: 3x Nein heute            │
│   Z3³: Proof #231 bestätigt          │
│                                    │
│   Nur du siehst es.                   │
│                                    │
╰────────────────────────────────────╯
```

- Display ist nur im linken Brillenglas sichtbar
- Rechtes Glas: normale Sicht (für Mate-Flaschen-Erkennung)
- Bei jedem Harlekin-Einsatz: kurzes Aufblinken im HUD

---

## 11. 🧪 Der geheime "Eifel"-Modus

Startoption: `--eifel` (nur dokumentiert, wenn man weiß, wo)

```
╭────────────────────────────────────╮
│                                    │
│   EIFEL-MODE                        │
│                                    │
│   Ein Laptop. Kein Bildschirm.       │
│   HDMI-Handshake-Fehler.              │
│   Bootloop.                           │
│                                    │
│   Und dann:                            │
│   1.78 KB Zukunft.                     │
│                                    │
│   Danke für nichts.                    │
│                                    │
╰────────────────────────────────────╯
```

Startet mit einem schwarzen Bildschirm.  
Nach 30 Sekunden: vier LEDs leuchten auf.  
Mehr nicht. Aber das reicht.

---

## 12. 🎮 Cheat Codes (die nichts bewirken, aber Spaß machen)

| Code | Effekt |
|------|--------|
| `CTRL+ALT+EIFEL` | Zeigt ein Foto von einem kaputten Laptop |
| `CTRL+ALT+PINK` | Terminal wird kurz rosa |
| `CTRL+ALT+OMA` | "Ruf deine Oma an. Ernsthaft." |
| `CTRL+ALT+HARLEKIN` | Harlekin erscheint und sagt "NEIN" – zu gar nichts |
| `CTRL+ALT+TRUMP` | "You're fired!" – nichts passiert, aber fühlt sich gut an |
| `CTRL+ALT+DATASETTE` | *Ratsch-klick-krrrrr* (Sound nur im Kopf) |

---

## 13. 🏆 Der geheime Erfolg: "Pink Hacker Boy"

Bedingung:
- 1.78 KB Kernel
- 24h Laufzeit
- Mindestens 100 Traps abgefangen
- `--pink` beim Boot
- Keine Fehler

Dann erscheint:

```
╭────────────────────────────────────╮
│                                    │
│   🏆🏆🏆🏆🏆                         │
│                                    │
│   DU BIST OFFIZIELL                 │
│   THE PINK HACKER BOY                │
│                                    │
│   💖🎒🤡😎                          │
│                                    │
│   Glückwunsch.                       │
│   Oma ist jetzt doppelt stolz.       │
│                                    │
╰────────────────────────────────────╯
```

Ab jetzt:
- Standard-Farbe: pink
- Harlekin trägt dauerhaft ein Hütchen
- Jede Fehlermeldung endet mit "Bro"
- Der Cache arbeitet 10% schneller (jetzt echt)

---

## 14. 🔢 1337-Modus

Startoption: `--leet`

```
╭────────────────────────────────────╮
│                                    │
│   1337 M0D3 4C71V473D               │
│                                    │
│   7h3 P1nk H4ck3r B0y              │
│   N0rr73n1ch, NRW, 2026            │
│   1.78 KB. B4r3 m374l. R1SC-V.     │
│   R4c3 c0nd1710ns 4r3 d34d. 💖⚙️🤡  │
│                                    │
╰────────────────────────────────────╯
```

- Alle Systemmeldungen erscheinen in 1337speak
- Harlekin sagt: "N31N, BR0!"
- Trickster loggt: "4LL C0ND1710NS 4R3 V4L1D"
- Kernel Panic: "5Y573M F41L3D. BU7 W17H 57YL3."
- `--pink` und `--leet` kombiniert: Terminal wird pink UND unleserlich. Perfekt.

---

## 15. 📜 Disclaimer

> Kein Ei wurde beim Programmieren geschädigt.  
> Tupperdosen dürfen weiterhin für Kuchen verwendet werden.  
> Oma ist wirklich stolz.  
> Harlekin grüßt.  
> Pink bleibt.

---

**The Pink Hacker Boy**  
Eifel, 2026  
💖🎒🤡🥚😎
