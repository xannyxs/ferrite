#include <stddef.h>

void *ft_memccpy(void *dest, const void *src, int c, size_t n) {
  unsigned char *ds;
  const unsigned char *sr;
  size_t i;

  sr = (const unsigned char *)src;
  ds = (unsigned char *)dest;
  i = 0;
  while (i < n) {
    ds[i] = sr[i];
    if (sr[i] == (unsigned char)c)
      return (dest + i + 1);
    i++;
  }
  return (NULL);
}
