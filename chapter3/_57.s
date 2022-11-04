	.file	"_57.c"
	.text
	.globl	funct3
	.type	funct3, @function
funct3:
.LFB0:
	.cfi_startproc
	vxorps	%xmm1, %xmm1, %xmm1
	vmovsd	%xmm0, %xmm0, %xmm3
	vmovss	(%rdx), %xmm0
	vcvtsi2sdl	(%rdi), %xmm1, %xmm2
	vcomisd	%xmm3, %xmm2
	jb	.L6
	vcvtsi2ssq	%rsi, %xmm1, %xmm1
	vaddss	%xmm0, %xmm1, %xmm1
	vaddss	%xmm0, %xmm1, %xmm0
.L4:
	vcvtss2sd	%xmm0, %xmm0, %xmm0
	ret
.L6:
	vcvtsi2ssq	%rsi, %xmm1, %xmm1
	vmulss	%xmm0, %xmm1, %xmm0
	jmp	.L4
	.cfi_endproc
.LFE0:
	.size	funct3, .-funct3
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
