    .section .init
    .global _init
_init:
    .arm
    ldr sp, =__sp_usr
    blx (_start)

    .section .header,#alloc
    .global _header
_header:
    .arm
    b (_init)

    .section .iwram
    .global __usr_irq_handler
__usr_irq_handler:
    .arm
    ldr r12, =_irq_handler
    bx r12
