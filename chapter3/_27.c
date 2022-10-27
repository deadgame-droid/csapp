#include <stdio.h>

long fun_b(unsigned long x) {
    long val = 0;
    long i;
    for (i = 64; i >= 1; i--) {
        unsigned long t = x;
        t &= 1;
        val += val;
        val |= t;
        x >>= 1;
    }
    return val;
}

int main() {
    long res = fun_b(0x8000000000000001);
    printf("res: 0x%lx\n", res);
}