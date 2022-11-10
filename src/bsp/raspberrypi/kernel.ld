__rpi_phy_dram_start_addr = 0; /* 内核栈区起始点 */
__rpi_phy_binary_load_addr = 0x80000; /* 数据区和代码区域起始点 */

ENTRY(__rpi_phy_binary_load_addr)


/* Flags:
 *     4 == R
 *     5 == RX
 *     6 == RW
 */
PHDRS
{
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code PT_LOAD FLAGS(5);
    segment_data PT_LOAD FLAGS(6);
}

SECTIONS
{
    . = 0x2000000;

    .boot_core_stack (NOLOAD): /* 定义内核栈段 */
        {
            . += __rpi_phy_binary_load_addr;
            __boot_core_stack_end_exclusive = .;
        } :segment_boot_core_stack

    __binary_nonzero_start = .;
    .text : /* 定义代码段 */
        {
            KEEP(*(.text._start))
            *(.text._start_arguments)
            *(.text._start_rust)
            *(.text*)
        } :segment_code

    .rodata : ALIGN(8) {*(.rodata*)} :segment_code

    .data : {*(.data*)} :segment_data

    __binary_nonzero_end_exclusive = .;

    .bss (NOLOAD) : ALIGN(16)
        {
            __bss_start = .;
            *(.bss*);
            . = ALIGN(16);
            __bss_end_exclusive = .;
        }

    .got : { *(.got*) }
    ASSERT(SIZEOF(.got) == 0, "Relocation support not expected")

    /DISCARD/ : { *(.comment*) }
}