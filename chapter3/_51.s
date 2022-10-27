	.file	"_51.c"
	.text
	.globl	cvt
	.type	cvt, @function
cvt:
.LFB0:
	.cfi_startproc
	cvtsd2ss	%xmm0, %xmm0
	ret
	.cfi_endproc
.LFE0:
	.size	cvt, .-cvt
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
