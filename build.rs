fn main() {
    println!("cargo:rerun-if-changed=target/entry.o");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rustc-link-arg=target/entry.o");
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rustc-link-arg=--oformat=binary");
}
