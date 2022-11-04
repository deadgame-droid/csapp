	.file	"_62.c"
	.text
	.globl	switch3
	.type	switch3, @function
switch3:
.LFB0:
	.cfi_startproc
	cmpl	$4, %edx
	ja	.L9
	movl	%edx, %edx
	jmp	*.L4(,%rdx,8)
	.section	.rodata
	.align 8
	.align 4
.L4:
	.quad	.L8
	.quad	.L7
	.quad	.L6
	.quad	.L5
	.quad	.L3
	.text
.L3:
	movl	$27, %eax
	ret
.L8:
	movq	(%rsi), %rax
	movq	(%rdi), %rdx
	movq	%rdx, (%rsi)
	ret
.L7:
	movq	(%rsi), %rax
	addq	(%rdi), %rax
	movq	%rax, (%rdi)
	ret
.L6:
	movq	$59, (%rdi)
	movq	$0, (%rsi)
	movl	$0, %eax
	ret
.L5:
	movq	(%rsi), %rax
	movq	%rax, (%rdi)
	movl	$27, %eax
	ret
.L9:
	movl	$12, %eax
	ret
	.cfi_endproc
.LFE0:
	.size	switch3, .-switch3
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
