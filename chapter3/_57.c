double funct3(int *ap, double b, long c, float *dp) {
    float xmm1 = *dp;
    if (*ap < b) {
        xmm1 *= c;
    } else {
        xmm1 += xmm1 + c;
    }
    return xmm1;
}