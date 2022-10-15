	.file	"_38.c"
	.text
	.globl	sum_element
	.type	sum_element, @function
sum_element:
.LFB0:
	.cfi_startproc
	leaq	0(,%rdi,8), %rdx
	subq	%rdi, %rdx
	addq	%rsi, %rdx
	leaq	(%rsi,%rsi,4), %rax
	addq	%rdi, %rax
	movq	Q(,%rax,8), %rax
	addq	P(,%rdx,8), %rax
	ret
	.cfi_endproc
.LFE0:
	.size	sum_element, .-sum_element
	.globl	Q
	.bss
	.align 32
	.type	Q, @object
	.size	Q, 280
Q:
	.zero	280
	.globl	P
	.align 32
	.type	P, @object
	.size	P, 280
P:
	.zero	280
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
