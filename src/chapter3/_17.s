	.file	"_17.c"
	.text
	.globl	gotodiff_se
	.type	gotodiff_se, @function
gotodiff_se:
.LFB0:
	.cfi_startproc
	cmpq	%rdi, %rsi
	jle	.L2
	movq	%rsi, %rax
	subq	%rdi, %rax
	ret
.L2:
	movq	%rdi, %rax
	subq	%rsi, %rax
	ret
	.cfi_endproc
.LFE0:
	.size	gotodiff_se, .-gotodiff_se
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
