#include<stdio.h>
// 溢出返回0
int tmult_ok(int x, int y) {
    int p = x * y;
    return !x || p/x == y;
}

int tmult_ok2(int x, int y) {
    long p = (long)x * (long)y;
    long real_p = (long)(int) p;
    if (p == real_p) {
        return 1;
    }else {
        return 0;
    }
}

int main() {
    int x = 2133333333;
    int y = 2222222222;
    int res = tmult_ok(x,y);
    int res2 = tmult_ok2(x,y);
    printf("%x * %x = %x, %d\n",x,y,x*y,res);
    printf("%x * %x = %x, %d\n",x,y,x*y,res2);
}