long A[8][5][13];

long store_ele(long i, long j, long k, long *dest) {
    *dest = A[i][j][k];
    return sizeof(A);
}
