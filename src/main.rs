#![no_std]
#![no_main]

#[no_mangle]
pub unsafe extern "C" fn rust_entry() {
    print("Hi from Rust!");
    loop {}
}

unsafe fn print(s: &str) {
    const TTY: u8 = 0xE;

    for &ch in s.as_bytes() {
        core::arch::asm!(
            "int 10h",
            in("ah") TTY,
            in("al") ch
        );
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
