#include <stddef.h>

int memcmp(const void *s1, const void *s2, size_t n) {
  const unsigned char *ucs1;
  const unsigned char *ucs2;

  ucs1 = (const unsigned char *)s1;
  ucs2 = (const unsigned char *)s2;
  if (n == 0 || s1 == s2)
    return (0);
  while (n) {
    n--;
    if (*ucs1 != *ucs2)
      return ((unsigned char)*ucs1 - (unsigned char)*ucs2);
    if (n) {
      ucs1++;
      ucs2++;
    }
  }
  return ((unsigned char)*ucs1 - (unsigned char)*ucs2);
}
