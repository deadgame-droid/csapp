long loop_while(long a, long b) {
    long result = 1;
    while (a < b) {
        result *= a + b;
        a += 1;
    }
    return result;
}