	.file	"_40.c"
	.text
	.globl	fix_set_diag_opt
	.type	fix_set_diag_opt, @function
fix_set_diag_opt:
.LFB0:
	.cfi_startproc
	movl	$0, %eax
.L2:
	movl	%esi, (%rdi,%rax,4)
	addq	$17, %rax
	cmpq	$272, %rax
	jne	.L2
	ret
	.cfi_endproc
.LFE0:
	.size	fix_set_diag_opt, .-fix_set_diag_opt
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
