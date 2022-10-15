long rfun(unsigned long x) {
    if (x == 0) {
        return 0;
    } else {
        unsigned long nx = x >> 2;
        long rv = rfun(nx);
        return rv + nx;
    }
}
