MEMORY {
    /* Virtual address mapped memory area */
    DRAM : ORIGIN = 0xffffffff80000000, LENGTH = 128M
}

/* Use virtual address is okay if you have an initial boot page */
PROVIDE(_stext = 0xffffffff80200000);
/* Modify this to provide bigger stack for each hart */
PROVIDE(_hart_stack_size = 128K);
/* Modify this to set max hart number */
PROVIDE(_max_hart_id = 3);
/* Modify this to add frame section size */
PROVIDE(_frame_size = 16384 * 4K);

/* Map the runtime regions into memory areas */
REGION_ALIAS("REGION_TEXT", DRAM);
REGION_ALIAS("REGION_RODATA", DRAM);
REGION_ALIAS("REGION_DATA", DRAM);
REGION_ALIAS("REGION_BSS", DRAM);
REGION_ALIAS("REGION_STACK", DRAM);
REGION_ALIAS("REGION_FRAME", DRAM);
