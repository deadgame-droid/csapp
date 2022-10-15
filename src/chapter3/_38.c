long P[5][7];
long Q[7][5];

long sum_element(long i, long j) {
    return P[i][j] + Q[j][i];
}