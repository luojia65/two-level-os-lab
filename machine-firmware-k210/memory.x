MEMORY {
    /* actual length is 6M; shrink to reserve for operating system */
    SRAM : ORIGIN = 0x80000000, LENGTH = 384K
}

REGION_ALIAS("REGION_TEXT", SRAM);
REGION_ALIAS("REGION_RODATA", SRAM);
REGION_ALIAS("REGION_DATA", SRAM);
REGION_ALIAS("REGION_BSS", SRAM);
REGION_ALIAS("REGION_HEAP", SRAM);
REGION_ALIAS("REGION_STACK", SRAM);

_max_hart_id = 1;
_hart_stack_size = 64K;
_heap_size = 128K;
