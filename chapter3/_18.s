	.file	"_18.c"
	.text
	.globl	test
	.type	test, @function
test:
.LFB0:
	.cfi_startproc
	leaq	(%rdi,%rsi), %rax
	addq	%rdx, %rax
	cmpq	$-3, %rdi
	jge	.L2
	cmpq	%rdx, %rsi
	jge	.L3
	movq	%rdi, %rax
	imulq	%rsi, %rax
	ret
.L3:
	movq	%rsi, %rax
	imulq	%rdx, %rax
	ret
.L2:
	cmpq	$2, %rdi
	jle	.L1
	movq	%rdi, %rax
	imulq	%rdx, %rax
.L1:
	ret
	.cfi_endproc
.LFE0:
	.size	test, .-test
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
