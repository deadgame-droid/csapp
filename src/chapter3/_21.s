	.file	"_21.c"
	.text
	.globl	test
	.type	test, @function
test:
.LFB0:
	.cfi_startproc
	testq	%rsi, %rsi
	jle	.L2
	movq	%rdi, %rdx
	andq	%rsi, %rdx
	movq	%rsi, %rax
	subq	%rdi, %rax
	cmpq	%rsi, %rdi
	cmovge	%rdx, %rax
	ret
.L2:
	leaq	0(,%rdi,8), %rax
	addq	%rsi, %rdi
	cmpq	$-1, %rsi
	cmovl	%rdi, %rax
	ret
	.cfi_endproc
.LFE0:
	.size	test, .-test
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
