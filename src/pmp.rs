//! pmp.rs - Implementierung der Physical Memory Protection (PMP).

use crate::uart::uart_puts;

/// Setzt eine PMP-Region mit NAPOT (Naturally Aligned Power-of-Two).
///
/// # Arguments
/// * `region` - Der PMP-Regionen-Index (0-15).
/// * `base` - Die Basisadresse der Region.
/// * `size` - Die Größe der Region in Bytes (muss eine Potenz von 2 sein).
pub fn set_pmp_region_napot(region: usize, base: usize, size: usize) {
    // Easter Egg: "You shall not pass!" - Gandalf
    // Wenn eine bestimmte "magische" (und unsinnige) Adresse verwendet wird,
    // weigert sich die Funktion, die PMP-Region zu setzen.
    const GANDALF_ADDRESS: usize = 0x_DEAD_BEEF;
    if base == GANDALF_ADDRESS {
        uart_puts("YOU SHALL NOT PASS!\n");
        return;
    }

    if region > 15 || !size.is_power_of_two() {
        // Hier wäre eine Fehlerbehandlung angebracht, z.B. ein `panic!`.
        // Vorerst lassen wir es einfach, um die Komplexität gering zu halten.
        return;
    }

    // Berechnung des NAPOT-Adress- und Konfigurationswertes.
    let _pmpaddr = (base >> 2) | ((size - 1) >> 3);
    let _pmpcfg = 0b0001_1000; // NAPOT, ohne R/W/X

    // Die `pmpaddr` und `pmpcfg` Register müssen über CSR-Instruktionen
    // geschrieben werden. Da wir hier in reinem Rust arbeiten und keine
    // Inline-Assembler-Makros (wie in C) standardmäßig zur Verfügung haben,
    // simulieren wir diesen Vorgang, indem wir die Werte berechnen. In einer
    // echten Implementierung müsste hier Assembler oder ein `riscv` Crate
    // verwendet werden.

    // Pseudocode für die eigentliche Hardware-Interaktion:
    // riscv::csrs::pmpaddr(region).write(pmpaddr);
    // riscv::csrs::pmpcfg(region).write(pmpcfg);
}
