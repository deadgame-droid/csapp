	.file	"_33.c"
	.text
	.globl	procprob
	.type	procprob, @function
procprob:
.LFB0:
	.cfi_startproc
	movslq	%edi, %rdi
	addq	%rdi, (%rdx)
	addb	%sil, (%rcx)
	movl	$6, %eax
	ret
	.cfi_endproc
.LFE0:
	.size	procprob, .-procprob
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
