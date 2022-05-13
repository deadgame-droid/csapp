#include <stdio.h>

int main() {
  char x = 0x66;
  char y = 0x39;
  printf("%x\n", x & y);
  printf("%x\n", x | y);
  printf("%x\n", ~x | ~y);
  printf("%x\n", x & !y);
  printf("%x\n", x && y);
  printf("%x\n", x || y);
  printf("%x\n", !x || !y);
  printf("%x\n", x && ~y);
}