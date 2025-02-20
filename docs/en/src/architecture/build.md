# Build Pipeline

It took me quite a long time to truly understand what the best approach is to compile my kernel. There were a few requirements I had for myself:

1. Rust code should be in rust files, ASM code should be in assembly files
2. Assembly should be compiled with `nasm`
3. The boot should start at `_start`, which is in an assembly file

I do not think it is practical to have 99% in Rust. Assembly is great for having full control of your CPU - you know exactly what your code does. Assembly should mainly focus on booting and attaching systems to the kernel, while Rust should make use of the resources it gets as a kernel.

## Build Process

This had a few unforeseen challenges. While compiling both Assembly & Rust separately is straightforward, combining them into one `bin` file was quite challenging, especially for testing.

Here's what happens when running `cargo run`:

1. First, it calls a Rust build script which:
   - Compiles the necessary assembly code with `nasm`
   - Links it together with Rust

2. Rust is compiled twice (in a way):
   - The kernel itself is compiled as a library
   - This library is then linked together with the assembly code into a binary file
   - The resulting binary file can be used to execute the kernel in `qemu`

3. After building, `cargo` uses a custom `runner.sh` script that:
   - Launches `qemu` to execute the kernel binary
   - Applies different flags based on the mode (e.g., `cargo test` runs without opening a window)

## Major Challenge

I encountered one significant challenge that took a long time to resolve. While `cargo run` worked without issues, `cargo test` would fail to find the `multiboot` flags.

Initially, I was convinced the issue was with `cargo`, not my linker. After using `dumpobj` to investigate, I discovered that `multiboot` wasn't present in the binary. The solution was adding a single line to my linker:

```
KEEP(*(.multiboot))
```

This forces the linker to keep the multiboot section in place, resolving the issue.
