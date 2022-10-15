	.file	"_35.c"
	.text
	.globl	rfun
	.type	rfun, @function
rfun:
.LFB0:
	.cfi_startproc
	movl	$0, %eax
	testq	%rdi, %rdi
	jne	.L8
	ret
.L8:
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	movq	%rdi, %rbx
	shrq	$2, %rbx
	movq	%rbx, %rdi
	call	rfun
	addq	%rbx, %rax
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE0:
	.size	rfun, .-rfun
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
