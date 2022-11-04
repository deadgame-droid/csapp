long switch_prob(long x, long n) {
    long result = x;
    switch (n) {
    case 0x3c:
    case 0x3e:
        result = 8 * x;
        break;
    case 0x3f:
        result = x >> 3;
        break;
    case 0x40:
        result = (x << 4) - x;
        result *= result;
        result += 0x4b;
        break;
    case 0x41:
        result = n * n + 0x4b;
        break;
    case 0x3d:
    default:
        result = x + 0x4b;
    }
    return result;
}

int main() {
    return 0;
}
