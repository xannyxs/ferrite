#include <stddef.h>

void *memcpy(void *dest, const void *src, size_t n) {
  void *original_dest = dest;

  if (!src || !dest) {
    return NULL;
  }

  const unsigned char *s = src;
  unsigned char *d = dest;

  while (n) {
    n--;
    *d = *s;
    d++;
    s++;
  }

  return original_dest;
}
