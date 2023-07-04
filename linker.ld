MEMORY {
    FLASH    (rx)  : ORIGIN = 0x0,    LENGTH = 144K
    RAM      (rxw) : ORIGIN = 0x7C00,  LENGTH = 144K
}

PHDRS {
   bootsector PT_LOAD;
   text PT_LOAD;
}

ENTRY(entry)

SECTIONS {
    .text.boot 0x7C00 : {
        target/entry.o(.text.*)
    } > FLASH AT > RAM : bootsector

    .text.section_signature (ADDR(.text.boot) + 510) : {
        SHORT(0xAA55);
    } > FLASH AT > RAM : bootsector

    .text.rust : {
        *(.text.*)
        . = ALIGN(512);
    } > FLASH AT > RAM : text = 0x00
    rust_code = LOADADDR(.text.rust);
}
