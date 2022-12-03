	.file	"_7.c"
	.text
	.section	.rodata.str1.1,"aMS",@progbits,1
.LC0:
	.string	"i = %ld, j = %ld\n"
	.text
	.globl	display
	.type	display, @function
display:
.LFB11:
	.cfi_startproc
	subq	$8, %rsp
	.cfi_def_cfa_offset 16
	movq	%rsi, %rdx
	movq	%rdi, %rsi
	movl	$.LC0, %edi
	movl	$0, %eax
	call	printf
	addq	$8, %rsp
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE11:
	.size	display, .-display
	.globl	main
	.type	main, @function
main:
.LFB12:
	.cfi_startproc
	subq	$8, %rsp
	.cfi_def_cfa_offset 16
    movq    %rsp, %rdi
    movq    $996, %r9
    pushq   %r9
    popq    %rsp
	movq	%rsp, %rsi
    movq    %rdi, %rsp
	call	display
	movl	$0, %eax
	addq	$8, %rsp
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE12:
	.size	main, .-main
	.ident	"GCC: (GNU) 11.3.1 20220421 (Red Hat 11.3.1-2)"
	.section	.note.GNU-stack,"",@progbits
