#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void *calloc(size_t nmemb, size_t size) {
    size_t bytes = nmemb * size;
    void *arr = malloc(bytes);
    return memset(arr, 0, bytes);
}

int main() {
    char *arr = (char *)calloc(8, 1);
    for (int i = 0; i < 8; i++) {
        printf("%d\n", arr[i]);
    }
}
