#include <stddef.h>

void *memset(void *str, int c, size_t len) {
  size_t i = 0;
  unsigned char *s = (unsigned char *)str;

  while (len) {
    len--;
    s[i] = c;
    i++;
  }

  return s;
}
