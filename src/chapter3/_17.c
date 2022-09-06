long gotodiff_se(long x, long y) {
    long result;
    if (y > x) {
        goto y_g_x;
    } else {
        result = x - y;
        return result;
    }

y_g_x:
    result = y - x;
    return result;
}