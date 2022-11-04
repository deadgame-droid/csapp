long decode2(long x, long y, long z) {
    y -= z;
    x *= y;
    y <<= 63;
    y >>= 63;
    y ^= x;
    return y;
}