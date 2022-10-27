	.file	"_12.c"
	.text
	.p2align 4
	.globl	uremdiv
	.type	uremdiv, @function
uremdiv:
.LFB0:
	.cfi_startproc
	movq	%rdx, %r8
	movq	%rdi, %rax
	xorl	%edx, %edx
	divq	%rsi
	movq	%rax, (%r8)
	movq	%rdx, (%rcx)
	ret
	.cfi_endproc
.LFE0:
	.size	uremdiv, .-uremdiv
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
