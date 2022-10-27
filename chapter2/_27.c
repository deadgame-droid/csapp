#include <stdio.h>
int uadd_ok(unsigned x, unsigned y) {
    unsigned s = x + y;
    if (s < x) {
        return 0;
    } else {
        return 1;
    }
}

int main() {
    unsigned x = -1;
    unsigned y = 1;
    printf("%d\n", uadd_ok(x, y));
}