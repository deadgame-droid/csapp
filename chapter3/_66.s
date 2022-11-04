	.file	"_66.c"
	.text
	.globl	sum_col
	.type	sum_col, @function
sum_col:
.LFB0:
	.cfi_startproc
	movq	%rdi, %rax
	salq	$5, %rax
	addq	$8, %rax
	leaq	(%rdi,%rdi,2), %r8
	testq	%r8, %r8
	jle	.L4
	leaq	0(,%rax,8), %rdi
	leaq	(%rsi,%rdx,8), %rdx
	movl	$0, %ecx
	movl	$0, %eax
.L3:
	addq	(%rdx), %rcx
	addq	$1, %rax
	addq	%rdi, %rdx
	cmpq	%rax, %r8
	jne	.L3
.L1:
	movq	%rcx, %rax
	ret
.L4:
	movl	$0, %ecx
	jmp	.L1
	.cfi_endproc
.LFE0:
	.size	sum_col, .-sum_col
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
