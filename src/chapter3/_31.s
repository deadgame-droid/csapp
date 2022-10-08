	.file	"_31.c"
	.text
	.globl	switcher
	.type	switcher, @function
switcher:
.LFB0:
	.cfi_startproc
	cmpq	$7, %rdi
	ja	.L6
	jmp	*.L4(,%rdi,8)
	.section	.rodata
	.align 8
	.align 4
.L4:
	.quad	.L7
	.quad	.L6
	.quad	.L3
	.quad	.L6
	.quad	.L8
	.quad	.L5
	.quad	.L6
	.quad	.L3
	.text
.L5:
	movq	%rsi, %rdx
	xorq	$15, %rdx
.L7:
	leaq	112(%rdx), %rsi
.L6:
	movq	%rsi, (%rcx)
	ret
.L3:
	addq	%rdx, %rsi
	salq	$2, %rsi
	jmp	.L6
.L8:
	movq	%rdi, %rsi
	jmp	.L6
	.cfi_endproc
.LFE0:
	.size	switcher, .-switcher
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
