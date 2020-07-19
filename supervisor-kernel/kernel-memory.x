MEMORY {
    /* Virtual address mapped memory area */
    DRAM : ORIGIN = 0xffffffff80000000, LENGTH = 4M 
}

/* Use virtual address is okay if you have an initial boot page */
_stext = 0xffffffff80200000;
/* Modify this to provide bigger stack for each hart */
_hart_stack_size = 32K;
/* Modify this to set max hart number */
_max_hart_id = 7;
/* Modify this to add frame section size */
_heap_size = 512K;

/* Map the runtime regions into memory areas */
REGION_ALIAS("REGION_TEXT", DRAM);
REGION_ALIAS("REGION_RODATA", DRAM);
REGION_ALIAS("REGION_DATA", DRAM);
REGION_ALIAS("REGION_BSS", DRAM);
REGION_ALIAS("REGION_HEAP", DRAM);
REGION_ALIAS("REGION_STACK", DRAM);
