#![feature(
    pointer_byte_offsets,
    const_pointer_byte_offsets,
    layout_for_ptr,
    const_size_of_val_raw
)]
#![no_std]
#![no_main]

mod gdt;

#[no_mangle]
pub unsafe extern "C" fn rust_entry() {
    enable_a20();
    loop {}
}

unsafe fn enable_a20() {
    let even = &mut *(0x1FFFFF as *mut u32);
    let odd = &mut *(0x0FFFFF as *mut u32);
    *odd = 0x00C0FFEE;
    *even = 0xDEADBEEF;

    let is_enabled = *even != *odd;
    if is_enabled {
        return;
    }

    let resp = x86::io::inb(0x92) | 0x02;
    x86::io::outb(0x92, resp);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
