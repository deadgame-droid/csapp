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

int tsub_ok_err(int x, int y) { return tadd_ok(x, -y); }
int tsub_ok(int x, int y) { return tadd_ok(x + 1, -(y + 1)); }
int tsub_ok1(int x, int y) {
    // ~y + 1 == -y is true
    if (x > 0 && y < 0 && ~y + 1 == y) {
        return 0;
    } else {
        return tadd_ok(x, -y);
    }
}
int main() {
    int x = 1;
    int y = -2147483648;
    printf("x - y : %s\n", tsub_ok_err(x, y) == 0 ? "overflow" : "ok");
    printf("x - y : %s\n", tsub_ok(x, y) == 0 ? "overflow" : "ok");
    printf("x - y : %s\n", tsub_ok1(x, y) == 0 ? "overflow" : "ok");
}