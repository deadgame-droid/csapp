#define NR(n) (3 * n)
#define NC(n) (32 * n + 8)

long sum_col(long n, long A[NR(n)][NC(n)], long j) {
    long i;
    long result = 0;
    for (i = 0; i < NR(n); i++) {
        result += A[i][j];
    }
    return result;
}
