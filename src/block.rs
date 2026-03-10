use crate::cond::{id, CondMask};
use crate::state::CoreState;
use crate::uart::uart_puts;

pub struct Block {
    pub name: &'static str,
    pub conditions: CondMask,
    pub body: fn(&mut CoreState),
}

fn block_pmp_ok(state: &mut CoreState) {
    uart_puts("  [block] pmp ok\n");
    state.set_cond(id::BOOT_DONE);
}

fn block_boot_done(_state: &mut CoreState) {
    uart_puts("  [block] boot done\n");
    // EASTER EGG: US-Hymne nach erfolgreichem Booten
    // "My country, 'tis of thee, Sweet land of liberty, of thee I sing;"
    uart_puts("\nMy country, 'tis of thee,\nSweet land of liberty,\nOf thee I sing;\n\n");
}

fn block_timer_tick(_state: &mut CoreState) {
    uart_puts("  [block] tick\n");
}

pub static BLOCKS: &[Block] = &[
    Block {
        name: "pmp_ok",
        conditions: CondMask::from_bit(id::PMP_OK),
        body: block_pmp_ok,
    },
    Block {
        name: "boot_done",
        conditions: CondMask::from_bit(id::BOOT_DONE),
        body: block_boot_done,
    },
    Block {
        name: "timer_tick",
        conditions: CondMask::from_bit(id::TIMER_TICK),
        body: block_timer_tick,
    },
];

pub fn run_blocks(blocks: &[Block], state: &mut CoreState) {
    for block in blocks {
        if state.conds.contains_all(block.conditions) {
            (block.body)(state);
        }
    }
}
