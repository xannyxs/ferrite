	; memset.asm - Optimized implementation of memset
	; Function: void *memset(void *s, int c, size_t n)
	; Params:
	; - s: Pointer to memory block [esp+4]
	; - c: Value to set (only low byte is used) [esp+8]
	; - n: Number of bytes to set [esp+12]
	; Return: Pointer to memory block (same as s)

	global  memset
	section .text

memset:
	push ebp
	mov  ebp, esp
	push edi

	mov   edi, [ebp+8]; s (destination)
	movzx eax, byte [ebp+12]; c (fill byte)
	mov   ecx, [ebp+16]; n (count)

	;   Save original destination for return value
	mov edx, edi

	;    Check if count is zero
	test ecx, ecx
	jz   .done

	;   For small counts, just use byte fill
	cmp ecx, 4
	jb  .byte_fill

	;    For larger counts with aligned pointer, optimize with DWORD fill
	test edi, 3
	jnz  .byte_fill

	;   Expand the byte value to fill all 4 bytes of eax
	mov ah, al
	mov edx, eax
	shl edx, 16
	or  eax, edx

	;   4-byte aligned fill
	mov edx, ecx
	shr ecx, 2
	rep stosd

	;   Handle remaining bytes
	mov ecx, edx
	and ecx, 3
	jz  .done

.byte_fill:
	rep stosb

.done:
	mov eax, [ebp+8]

	pop edi
	pop ebp
	ret
