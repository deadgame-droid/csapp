	.file	"_43.c"
	.text
	.globl	get
	.type	get, @function
get:
.LFB0:
	movq	(%rdi), %rax
	movl	(%rdi,%rax,4), %eax
	movl	%eax, (%rsi)
	ret
.LFE0:
	.size	get, .-get
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
