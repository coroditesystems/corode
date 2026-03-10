.section .text
.global __trap_vector
.align 2

__trap_vector:
    # 1. Save minimal context to ensure a safe call to Rust.
    addi sp, sp, -16
    sd   ra, 0(sp)
    sd   t0, 8(sp)

    # 2. Call the non-returning Rust trap handler.
    call rust_handle_trap

    # 3. Fallback: Should never be reached if rust_handle_trap is '-> !'.
1:
    wfi
    j 1b
