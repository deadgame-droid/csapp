#include <stdio.h>
int bad_int_size_is_32() {
    int set_msb = 1 << 31;
    int beyond_msb = 1 << 32;
    return set_msb && !beyond_msb;
}

int at_least_32_int_size_is_32() {
    int set_msb = 1 << 31;
    // 这是我想出来的必不移位超过int自身的bits数
    int beyond_msb = ~set_msb + 1;
    return set_msb && beyond_msb == set_msb;
}

int at_least_16_int_size_is_32() {
    // 参考了某个天才的想法，我是想不出来(笑)。这我觉得是错误的，但我暂时想不出答案。
    int set_msb = 1 << 15 << 15 << 1;
    int beyond_msb = 1;
    return set_msb && !beyond_msb;
}
int main() {
    printf("%d\n", at_least_32_int_size_is_32());

    return 1;
}