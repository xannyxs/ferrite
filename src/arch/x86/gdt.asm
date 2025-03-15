section .text
global  gdt_flush

gdt_flush:
	mov  eax, [esp + 4]
	lgdt [eax]

	;   Enable protected mode
	mov eax, cr0
	or  eax, 1
	mov cr0, eax

	;   Set up segments
	mov eax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax

	;   Far jump to flush pipeline and load CS
	jmp 0x08:.flush

.flush:
	ret
