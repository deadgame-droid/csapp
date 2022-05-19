#include <stdio.h>
int tadd_ok(int x, int y) {
  int s = x + y;
  if (x > 0 && y > 0 && s <= 0) {
    // 正溢出
    return 0;
  } else if (x < 0 && y < 0 && s >= 0) {
    // 负溢出
    return 0;
  } else {
    return 1;
  }
}

int tsub_ok(int x, int y) { return tadd_ok(x, -y); }
int main() {
  //   int x = 2147483647;
  //   int y = -2147483648;
  int x = 2147483647;
  int y = -2147483647;
  printf("x - y : %s\n", tsub_ok(x, y) == 0 ? "overflow" : "ok");
}