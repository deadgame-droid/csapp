// y86-64
	.file	"_4.c"
	.text
	.globl	rsum
	.type	rsum, @function
rsum:
	irmovq	$0, %rax
	andq	%rsi, %rsi
	jg	    loop
	ret
loop:
	pushq	%rbx
	mrmovq	(%rdi), %rbx
    irmovq  $1, %r8
	subq	%r8, %rsi
    irmovq  $8, %r9
    addq    %rdi, $r9
	call	rsum
	addq	%rbx, %rax
	popq	%rbx
	ret
