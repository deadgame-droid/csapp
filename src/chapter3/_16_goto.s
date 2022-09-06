	.file	"_16.c"
	.text
	.globl	goto_cond
	.type	goto_cond, @function
goto_cond:
.LFB0:
	.cfi_startproc
	testq	%rsi, %rsi
	je	.L1
	cmpq	%rdi, (%rsi)
	jge	.L1
	movq	%rdi, (%rsi)
.L2:
.L1:
	ret
	.cfi_endproc
.LFE0:
	.size	goto_cond, .-goto_cond
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
