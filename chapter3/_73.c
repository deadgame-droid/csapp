#include <assert.h>
#include <stdio.h>

typedef enum { NEG, ZERO, POS, OTHER } range_t;

range_t find_range(float x) {
    __asm__("vxorps %xmm1, %xmm1, %xmm1\n\t"
            "ucomiss %mm1, %mm0\n\t"
            "jp .O\n\t"
            "ja .P\n\t"
            "jl .N\n\t"
            "je .Z\n"
            ".O: \n\t"
            "movl $4, %eax\n\t"
            "ret\n"
            ".P: \n\t"
            "movl $3, %eax\n\t"
            "ret\n"
            ".N: \n\t"
            "movl $0, %eax\n\t"
            "ret\n"
            ".Z: \n\t"
            "movl $1, %eax\n\t"
            "ret\n\t");
}

int main(int argc, char *argv[]) {
    range_t n = NEG, z = ZERO, p = POS, o = OTHER;
    assert(o == find_range(0.0 / 0.0));
    assert(n == find_range(-2.3));
    assert(z == find_range(0.0));
    assert(p == find_range(3.33));
    return 0;
}
