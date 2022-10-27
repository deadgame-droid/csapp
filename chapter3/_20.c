#define OP /

long arith(long x) {
    // 除法是向零舍入，负数需要加上偏置值
    return x OP 8;
}