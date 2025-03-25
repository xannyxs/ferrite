#include <stddef.h>

int memcmp(const void *s1, const void *s2, size_t n) {
  const unsigned char *bytes1 = (const unsigned char *)s1;
  const unsigned char *bytes2 = (const unsigned char *)s2;

  if (n == 0 || s1 == s2)
    return 0;

  while (n--) {
    if (*bytes1 != *bytes2) {
      return *bytes1 - *bytes2;
    }

    bytes1++;
    bytes2++;
  }

  return 0;
}
