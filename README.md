The talk went over the process from getting a Master Boot Record written in assembly to bootstrap Rust code.

# Building

This projects uses NASM for assembling the MBR and QEMU for emulating the x86
system.

Build and run:

```bash
make all
make run
```
