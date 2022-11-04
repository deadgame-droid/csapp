	.file	"_63.c"
	.text
	.globl	switch_prob
	.type	switch_prob, @function
switch_prob:
.LFB0:
	.cfi_startproc
	leaq	-60(%rsi), %rax
	cmpq	$5, %rax
	ja	.L2
	jmp	*.L4(,%rax,8)
	.section	.rodata
	.align 8
	.align 4
.L4:
	.quad	.L7
	.quad	.L2
	.quad	.L7
	.quad	.L6
	.quad	.L5
	.quad	.L3
	.text
.L7:
	leaq	0(,%rdi,8), %rax
	ret
.L6:
	movq	%rdi, %rax
	sarq	$3, %rax
	ret
.L5:
	movq	%rdi, %rax
	salq	$4, %rax
	subq	%rdi, %rax
	imulq	%rax, %rax
	addq	$75, %rax
	ret
.L3:
	imulq	%rsi, %rsi
	leaq	75(%rsi), %rax
	ret
.L2:
	leaq	75(%rdi), %rax
	ret
	.cfi_endproc
.LFE0:
	.size	switch_prob, .-switch_prob
	.globl	main
	.type	main, @function
main:
.LFB1:
	.cfi_startproc
	movl	$0, %eax
	ret
	.cfi_endproc
.LFE1:
	.size	main, .-main
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
