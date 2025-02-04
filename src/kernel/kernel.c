#include <stddef.h>

#include <kernel/tty.h>

void kernel_main(void) {
  terminal_init();
  terminal_writestring("Hello, Kernel world!\nI am shown in a VM\n");
}
