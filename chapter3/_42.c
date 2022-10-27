struct ELE {
    long v;
    struct ELE *p;
};

long fun(struct ELE *ptr) {
    long res = 0;
    while (ptr != 0) {
        res += ptr->v;
        ptr = ptr->p;
    }
    return res;
}
