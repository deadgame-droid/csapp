#include <limits.h>
#include <stdio.h>

// 只使用位运算求绝对值
unsigned int abs_value(int value) {
    // -1 or 1
    int neg = ((value >> 31) << 1) + 1;

    // 因为-INT_MIN的绝对值在int中溢出，只能使用unsigned，这里是隐式转化
    return neg * value;
}

int main() {
    printf("%u\n", abs_value(1));
}
