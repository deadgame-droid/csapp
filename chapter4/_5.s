# long abs_sum(long *start, long count)
# start in %rdi, count in %rsi
abs_sum:
    irmovq $8, %r8
    irmovq $1, %r9
    xorq %rax, %rax
    andq %rsi, %rsi
    jmp test
loop:
    mrmovq (%rdi), %r10
    # set CC
    xorq %r11, %r11
    # r11 = -r10
    subq %r10, %r11
    jle pos         # r10>=0 , jmp to pos
    rrmovq %r11, %r10 # r10 = - r10;
pos:
    addq %r10, %rax
    addq %r8, %rdi
    subq %r9, %rsi
test:
    jne loop
    ret
