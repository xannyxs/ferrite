#include <stddef.h>

void *memset(void *str, int c, size_t len) {
  size_t i;
  unsigned char *s;

  s = (unsigned char *)str;
  i = 0;
  while (len) {
    len--;
    s[i] = c;
    i++;
  }
  return (s);
}
