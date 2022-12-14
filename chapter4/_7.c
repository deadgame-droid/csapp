#include <stdio.h>

// 用于生成汇编，然后再手动修改汇编代码，打印出%rsp的两个地址
void display(long i, long j) {
    printf("i = %ld, j = %ld\n", i, j);
}

int main() {
    long i = -1;
    long j = -2;
    display(i, j);
    return 0;
}
