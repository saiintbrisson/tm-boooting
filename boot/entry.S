BITS 16

SECTION .text.boot

GLOBAL entry
GLOBAL drive_number
EXTERN rust_entry
EXTERN rust_code
EXTERN GDTR
EXTERN RUST_CODE_SEG_OFFSET

entry:
    mov [drive_number], dl

    xor ax, ax
    mov bx, ax
    mov cx, ax
    mov dx, ax
    mov ds, ax
    mov es, ax
    mov ss, ax

    mov bp, 0x9000
    mov sp, bp

    call read_from_disk

    cli
    lgdt [GDTR]
    mov eax, cr0
    or al, 1
    mov cr0, eax

    mov ax, [RUST_CODE_SEG_OFFSET]
    mov [rust_entry_addr + 2], ax
    jmp far [rust_entry_addr]

read_from_disk:
    pusha

    mov ah, 0x2
    mov al, 63
    mov cl, 2
    mov dl, [drive_number]
    mov bx, rust_code

    int 0x13

    popa
    ret

.print_error:
    mov si, .error_msg
    mov ah, 0xE
.loop:
    lodsb
    test al, al
    jz .print_end
    int 0x10
    jmp .loop
.print_end:
    jmp $
.error_msg: db 'Drive read error', 0

rust_entry_addr:
    dw rust_entry ; ip
    dw 0 ; cs

drive_number: db 0