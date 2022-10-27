	.file	"_27.c"
	.text
	.globl	fun_b
	.type	fun_b, @function
fun_b:
.LFB11:
	.cfi_startproc
	movl	$64, %edx
	movl	$0, %eax
.L2:
	addq	%rax, %rax
	movq	%rdi, %rcx
	andl	$1, %ecx
	orq	%rcx, %rax
	shrq	%rdi
	subq	$1, %rdx
	jne	.L2
	ret
	.cfi_endproc
.LFE11:
	.size	fun_b, .-fun_b
	.section	.rodata.str1.1,"aMS",@progbits,1
.LC0:
	.string	"res: 0x%lx\n"
	.text
	.globl	main
	.type	main, @function
main:
.LFB12:
	.cfi_startproc
	subq	$8, %rsp
	.cfi_def_cfa_offset 16
	movl	$1, %edi
	call	fun_b
	movq	%rax, %rsi
	movl	$.LC0, %edi
	movl	$0, %eax
	call	printf
	movl	$0, %eax
	addq	$8, %rsp
	.cfi_def_cfa_offset 8
	ret
	.cfi_endproc
.LFE12:
	.size	main, .-main
	.ident	"GCC: (GNU) 11.2.1 20220127 (Red Hat 11.2.1-9)"
	.section	.note.GNU-stack,"",@progbits
