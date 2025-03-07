	; memcpy.asm - Optimized implementation of memcpy
	; Function: void *memcpy(void *dest, const void *src, size_t n)
	; Params:
	; - dest: Destination buffer pointer [esp+4]
	; - src: Source buffer pointer [esp+8]
	; - n: Number of bytes to copy [esp+12]
	; Return: Pointer to destination (same as dest)

	global  memcpy
	section .text

memcpy:
	push ebp
	mov  ebp, esp
	push esi
	push edi

	mov edi, [ebp+8]; dest
	mov esi, [ebp+12]; src
	mov ecx, [ebp+16]; n

	;   Save original destination for return value
	mov eax, edi

	;    Check if count is zero
	test ecx, ecx
	jz   .done

	;   Check for alignment possibilities
	cmp ecx, 4
	jb  .byte_copy

	;    Check alignment of both pointers
	mov  edx, esi
	or   edx, edi
	test dl, 3
	jnz  .byte_copy

	;   4-byte aligned copy
	shr ecx, 2
	rep movsd

	;   Handle remaining bytes
	mov ecx, [ebp+16]
	and ecx, 3
	jz  .done

.byte_copy:
	;   Byte-by-byte copy
	rep movsb

.done:
	pop edi
	pop esi
	pop ebp
	ret
