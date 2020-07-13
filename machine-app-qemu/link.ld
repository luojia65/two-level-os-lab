MEMORY {
    /* 存储单元的物理地址 */
    SRAM : ORIGIN = 0x80000000, LENGTH = 6M
}

PROVIDE(_stext = 0x80000000);
PROVIDE(_heap_size = 20K);

/* 目标架构 */
OUTPUT_ARCH(riscv)

/* 执行入口 */
ENTRY(_start)

SECTIONS
{
    /* .text 字段 */
    .text _stext : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
        _etext = .;
    } > SRAM

    /* .rodata 字段 */
    .rodata : ALIGN(4) {
        _srodata = .;
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
        . = ALIGN(4);
        _erodata = .;
    } > SRAM

    /* .data 字段 */
    .data : ALIGN(4) { 
        _sidata = LOADADDR(.data);
        _sdata = .;
        /* Must be called __global_pointer$ for linker relaxations to work. */
        PROVIDE(__global_pointer$ = . + 0x800);
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*)
        . = ALIGN(4);
        _edata = .;
    } > SRAM

    /* .bss 字段 */
    .bss (NOLOAD) : {
        _sbss = .;
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
        . = ALIGN(4);
        _ebss = .;
    } > SRAM

    .heap (NOLOAD) : {
        _sheap = .;
        . += _heap_size;
        . = ALIGN(4);
        _eheap = .;
    } > SRAM

    /* fictitious region that represents the memory available for the stack */
    .stack (NOLOAD) : {
        _estack = .;
        . = ALIGN(4);
        _sstack = .;
    } > SRAM

    /* Discard .eh_frame, we are not doing unwind on panic so it is not needed */
    /DISCARD/ :
    {
        *(.eh_frame .eh_frame_hdr);
    }
}
