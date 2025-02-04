#include <stddef.h>

void *memmove(void *dest, const void *src, size_t len) {
  char *str_dest;
  char *str_src;

  str_dest = (char *)dest;
  str_src = (char *)src;
  if (dest != NULL || src != NULL) {
    if (str_dest > str_src) {
      while (len--) {
        str_dest[len] = str_src[len];
      }
    } else {
      while (len--) {
        *str_dest = *str_src;
        str_dest++;
        str_src++;
      }
    }
  }
  return (dest);
}
