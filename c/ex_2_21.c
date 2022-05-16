#include <stdio.h>
int main() {
  unsigned int left = -2147483647 - 1u;
  int right = 2147483647;
  printf("%u < %u is %d\n", left, right, left < right);
}