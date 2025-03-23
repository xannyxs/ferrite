#ifndef LIBC_H
#define LIBC_H

#include <stddef.h>

void *memset(void *str, int c, size_t len);
int memcmp(const void *s1, const void *s2, size_t n);
void *memcpy(void *dest, const void *src, size_t n);

#endif /* LIBC_H */
