SECTIONS {
    .payload (NOLOAD) : ALIGN(4K) {
        _start_payload = .;
    } > SRAM
}
