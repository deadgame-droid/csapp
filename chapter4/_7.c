#include <stdio.h>

void display(long i, long j) {
    printf("i = %ld, j = %ld\n", i, j);
}

int main() {
    long i = -1;
    long j = -2;
    display(i, j);
    return 0;
}
