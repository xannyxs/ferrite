section .text
global  enable_paging

enable_paging:
	extern init_paging_directory

	push ebx
	call init_paging_directory
	mov  cr3, eax
	pop  ebx

	mov eax, cr0
	or  eax, 0x80000000
	mov cr0, eax

	ret
