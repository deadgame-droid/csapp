	.file	"_59.c"
	.text
	.globl	store_prod
	.type	store_prod, @function
store_prod:
.LFB0:
	.cfi_startproc
	movq	%rsi, %rax
	movq	%rdx, %rsi
	sarq	$63, %rsi
	movq	%rax, %rcx
	sarq	$63, %rcx
	imulq	%rdx, %rcx
	imulq	%rax, %rsi
	addq	%rsi, %rcx
	mulq	%rdx
	addq	%rcx, %rdx
	movq	%rax, (%rdi)
	movq	%rdx, 8(%rdi)
	ret
	.cfi_endproc
.LFE0:
	.size	store_prod, .-store_prod
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
