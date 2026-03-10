.section .text
.global __trap_vector
.align 2

__trap_vector:
    # This is the raw trap entry point set in mtvec.
    # It is kept minimal to ensure stability.
    # We only save the registers we need to make a safe call to a Rust function.
    # A full trap frame would save all registers (x1-x31), mepc, mcause, etc.
    # For this simple kernel, saving a few callers-saved registers is enough
    # to prevent the Rust compiler from tripping over unexpected changes.

    # 1. Save caller-saved registers that Rust might be using.
    # We save a few key ones: ra, t0-t2.
    addi sp, sp, -16
    sd   ra, 0(sp)  # Save return address
    sd   t0, 8(sp)  # Save temporary register t0

    # 2. Call the Rust handler.
    # The pointer to the saved registers could be passed in a0 if needed,
    # but our handler reads CSRs directly for simplicity.
    call rust_handle_trap

    # 3. Restore registers (although we halt in the handler).
    # This part is technically not reached, but good practice.
    ld   t0, 8(sp)
    ld   ra, 0(sp)
    addi sp, sp, 16

    # Return from trap (not reached in our case)
    mret
