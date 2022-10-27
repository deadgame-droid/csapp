// void cond(long a, long *p) {
//     if (p && (a > *p)) {
//         *p = a;
//     }
// }

void goto_cond(long a, long *p) {
    if (p == 0) {
        goto le;
    } else if (a <= *p) {
        goto le;
    } else {
        *p = a;
    }
le:
    return;
}