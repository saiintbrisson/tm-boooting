all: target/i386-unknown-none/release/boooting

target/entry.o:
	mkdir -p target
	nasm -f elf32 -o target/entry.o boot/entry.S
target/i386-unknown-none/release/boooting: target/entry.o
	cargo build --release

clean:
	cargo clean

run: all
	qemu-system-i386 -hda target/i386-unknown-none/release/boooting