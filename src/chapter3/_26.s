	.text
	.file	"_26.c"
	.globl	fun_a                           # -- Begin function fun_a
	.p2align	4, 0x90
	.type	fun_a,@function
fun_a:                                  # @fun_a
	.cfi_startproc
# %bb.0:
	xorl	%eax, %eax
	testq	%rdi, %rdi
	je	.LBB0_4
# %bb.1:
	movq	%rdi, %rcx
	.p2align	4, 0x90
.LBB0_2:                                # =>This Inner Loop Header: Depth=1
	xorq	%rdi, %rax
	shrq	%rcx
	cmpq	$1, %rdi
	movq	%rcx, %rdi
	ja	.LBB0_2
# %bb.3:
	andl	$1, %eax
.LBB0_4:
	retq
.Lfunc_end0:
	.size	fun_a, .Lfunc_end0-fun_a
	.cfi_endproc
                                        # -- End function
	.ident	"clang version 13.0.1 (Red Hat 13.0.1-1.el9)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
