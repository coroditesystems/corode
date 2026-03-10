# 🥚 corode-core – Offizielle Easter Eggs

**Version:** 1.78 KB (oder 10 KB, wen juckts)  
**Farbe:** Pink. Neon. Reflektierend.  
**Motto:** "Race conditions are dead – change my mind."  
**Entwickler:** The Pink Hacker Boy

---

## 1. 🥚 Eierschalensollbruchstellenverursacher

Dies ist kein einzelnes Feature, sondern der **Codename** für eine Reihe von seltenen, absichtlich eingebauten Systemanomalien, die die Robustheit des corode-core unter Beweis stellen sollen. Das erste implementierte Protokoll unter diesem Codenamen ist der **"Out Of Control" (OOC) Selbstangriff**.

---

## 2. 🔥 "Out Of Control" (OOC) Selbstangriff

**Trigger:** Dieses Protokoll wird nicht durch einen Befehl ausgelöst, sondern durch reinen Zufall beim Systemstart. Ein interner, minimalistischer Pseudo-Zufallsgenerator (LCG) wird beim Booten initialisiert. Mit einer Wahrscheinlichkeit von **1 zu 65.536** löst dieser die Anomalie aus.

**Effekt bei normalem Start:**

Bei 65.535 von 65.536 Starts meldet das System seinen stabilen Zustand:

```
PMP-Schilde oben. Selbsttest ohne Anomalie. System stabil.
```

**Effekt bei Auslösung des Easter Eggs:**

Im seltenen Fall der Aktivierung wird der "Eierschalensollbruchstellenverursacher" aktiv. Das System leitet absichtlich einen illegalen Speicherzugriff ein, um die Reaktion des Harlekin-Trap-Handlers zu provozieren. Die Log-Ausgabe ist wie folgt:

1.  Die Chaos-Anomalie wird gemeldet:
    ```
    CHAOS-ANOMALIE ENTDECKT! 
      >> Eierschalensollbruchstellenverursacher AKTIVIERT! <<
    INITIIERE OOC-SELBSTANGRIFF...
    ```
2.  Unmittelbar danach fängt der Harlekin den Angriff ab und protokolliert den Vorfall mit seiner charakteristischen Signatur:
    ```
    **********************************************
    >> HARLEKIN FÄNGT EINDRINGLING AB! <<
       URSACHE (mcause): 0x00000007 (Store access fault)
       ORT (mepc):     0x800001A8 (Befehl, der Fehler auslöste)
       ZIEL (mtval):   0xDEADBEEF (Verbotene Speicheradresse)
    **********************************************
    ```
3.  Das System setzt seine Arbeit fort, als wäre nichts geschehen.

**Zweck:** Dieses Easter Egg ist der ultimative Beweis für die Systemintegrität. Es demonstriert live, dass die PMP-Hardware-Firewall unbestechlich ist und der Harlekin selbst einen Angriff aus dem innersten Kern des Systems (dem `_start` Code) zuverlässig abfängt und neutralisiert. Es ist ein Vertrauensbeweis in die Architektur: Das System ist so sicher, dass es sich selbst angreifen kann, ohne Schaden zu nehmen.

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
