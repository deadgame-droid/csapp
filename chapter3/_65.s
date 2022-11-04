	.file	"_65.c"
	.text
	.globl	transpose
	.type	transpose, @function
transpose:
.LFB0:
	.cfi_startproc
	leaq	120(%rdi), %r10
	leaq	8(%rdi), %r9
	subq	$-128, %rdi
	movl	$0, %r8d
	jmp	.L2
.L3:
	movq	(%rax), %rcx
	movq	(%rdx), %rsi
	movq	%rsi, (%rax)
	movq	%rcx, (%rdx)
	addq	$8, %rax
	addq	$120, %rdx
	cmpq	%rdi, %rax
	jne	.L3
.L5:
	addq	$120, %r10
	addq	$8, %r9
	subq	$-128, %rdi
.L2:
	addq	$1, %r8
	cmpq	$15, %r8
	je	.L1
	movq	%r9, %rdx
	movq	%r10, %rax
	testq	%r8, %r8
	jg	.L3
	jmp	.L5
.L1:
	ret
	.cfi_endproc
.LFE0:
	.size	transpose, .-transpose
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
