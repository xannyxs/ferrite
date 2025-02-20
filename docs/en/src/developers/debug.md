# Debugging

Debugging is essential for finding bugs and truly understanding what your program does. While different programming languages have various debugging approaches, for low-level programs like kernels, `gdb` is an extremely powerful and useful tool.

Unlike regular programs where you can directly run `gdb` on the compiled binary, kernel debugging requires additional steps since we're running our code in a VM.

There are several debugging approaches:

1. Create logfiles
2. Print to the terminal (via QEMU)
3. Use `gdb`

The first two methods are similar, with the main difference being the output destination - file vs terminal. Choose based on your preference.

## GDB with QEMU

QEMU can be configured to wait for a GDB connection before executing any code, enabling debugging. Here's how to set it up:

1. Run `cargo debug` in the src/kernel directory (this is an alias for `debug = "run debug"`)
2. Cargo will build and run `runner.sh`, which adds the appropriate QEMU flags based on the mode (debug, test, or normal)
3. For debugging, the crucial flags are `-s -S`
4. A window will open & QEMU will wait for GDB to connect before starting execution

Connect GDB from another terminal:

```bash
gdb
(gdb) target remote localhost:1234
```

Load debug symbols:

```bash
(gdb) symbol-file <kernel bin>
```

Example debugging session:

```bash
gdb
(gdb) target remote localhost:1234
Remote debugging using localhost:1234
0x0000fff0 in ?? ()

(gdb) symbol-file kernel.b
Reading symbols from kernel.b...done.

(gdb) break kernel_main     # Add breakpoint to any kernel function
Breakpoint 1 at 0x101800: file kernel/kernel.c, line 12.

(gdb) continue # Qemu starts the kernel
Breakpoint 1, kernel_main (mdb=0x341e0, magic=0) at kernel/kernel.c:12
12      {
```

## Rust & Debugging

Rust's name mangling can make debugging challenging since function names may be modified. To ensure a function name remains unchanged for debugging purposes, add the following attribute:

```rust
#[no_mangle]
```

This prevents `rustc` from modifying the function name, making it easier to set breakpoints.

## Reference

- [OsDev Wiki - Kernel Debugging](https://wiki.osdev.org/Kernel_Debugging)
