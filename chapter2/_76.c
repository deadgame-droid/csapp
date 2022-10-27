#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void *calloc(size_t nmemb, size_t size) {
    size_t bytes = nmemb * size;
    int is_overflow = bytes < nmemb || bytes < size;
    if (bytes == 0 || is_overflow) {
        return NULL;
    }
    void *arr = malloc(bytes);
    return memset(arr, 0, bytes);
}

int main() {
    char *arr = (char *)calloc(0xffffffffffffffff, 2);
    if (arr == NULL) {
        printf("NULL\n");
        return -1;
    } else {
        for (int i = 0; i < 8; i++) {
            printf("%d\n", arr[i]);
        }
    }
}
