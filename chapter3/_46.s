	.file	"_46.c"
	.text
	.globl	gets
	.type	gets, @function
gets:
.LFB22:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	pushq	%rbx
	.cfi_def_cfa_offset 24
	.cfi_offset 3, -24
	subq	$8, %rsp
	.cfi_def_cfa_offset 32
	movq	%rdi, %rbp
	movq	%rdi, %rbx
.L2:
	movq	stdin(%rip), %rdi
	call	getc
	cmpl	$-1, %eax
	je	.L8
	cmpl	$10, %eax
	je	.L8
	addq	$1, %rbx
	movb	%al, -1(%rbx)
	jmp	.L2
.L8:
	cmpl	$-1, %eax
	jne	.L9
	movl	$0, %eax
	cmpq	%rbp, %rbx
	je	.L1
.L9:
	movb	$0, (%rbx)
	movq	%rbp, %rax
.L1:
	addq	$8, %rsp
	.cfi_def_cfa_offset 24
	popq	%rbx
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE22:
	.size	gets, .-gets
	.globl	get_line
	.type	get_line, @function
get_line:
.LFB23:
	.cfi_startproc
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset 3, -16
	subq	$16, %rsp
	.cfi_def_cfa_offset 32
	leaq	12(%rsp), %rdi
	call	gets
	leaq	12(%rsp), %rdi
	call	strlen
	leaq	1(%rax), %rdi
	call	malloc
	movq	%rax, %rbx
	testq	%rax, %rax
	je	.L11
	leaq	12(%rsp), %rsi
	movq	%rax, %rdi
	call	strcpy
.L11:
	movq	%rbx, %rax
	addq	$16, %rsp
	.cfi_def_cfa_offset 16
	popq	%rbx
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE23:
	.size	get_line, .-get_line
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
