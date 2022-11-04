	.file	"_60.c"
	.text
	.globl	loop
	.type	loop, @function
loop:
.LFB0:
	.cfi_startproc
	movl	%esi, %ecx
	movl	$1, %eax
	movl	$0, %edx
	jmp	.L2
.L3:
	movq	%rax, %r8
	andq	%rdi, %r8
	orq	%r8, %rdx
	salq	%cl, %rax
.L2:
	testq	%rax, %rax
	jne	.L3
	movq	%rdx, %rax
	ret
	.cfi_endproc
.LFE0:
	.size	loop, .-loop
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
