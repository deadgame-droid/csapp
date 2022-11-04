#include <stdio.h>
#define BUF_SIZE 5

void good_echo(void) {
    char buf[BUF_SIZE];
    /* function fgets is interesting */
    char *p = fgets(buf, BUF_SIZE, stdin);
    if (p == NULL) {
        return;
    }
    printf("%s", p);
}

int main(int argc, char *argv[]) {
    good_echo();
    return 0;
}
