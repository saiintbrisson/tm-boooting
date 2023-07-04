MEMORY {
    BOOTSECTOR (rx)  : ORIGIN = 0x7C00, LENGTH = 512
    BOOTLOADER (rxw) : ORIGIN = 0x7E00, LENGTH = 0x80000 - 0x7E00
}

ENTRY(entry)

SECTIONS {
    .text.boot : {
        target/entry.o(.text.*)
    } > BOOTSECTOR

    .text.section_signature (ADDR(.text.boot) + 510) : {
        SHORT(0xAA55);
    } > BOOTSECTOR

    .text.rust : {
        *(.text .text.*)
        *(.data .data.*)
        *(.rodata .rodata.*)
        *(.bss .bss.*)
        *(.got .got.plt)
        . = ALIGN(512);
    } > BOOTLOADER = 0x00
    rust_code = LOADADDR(.text.rust);

    /DISCARD/ : {
        *(.eh_frame*)
    }
}
