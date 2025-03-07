	; memcmp.asm - Optimized implementation of memcmp
	; Function: int memcmp(const void *s1, const void *s2, size_t n)
	; Params:
	; - s1: First buffer to compare [esp+4]
	; - s2: Second buffer to compare [esp+8]
	; - n: Number of bytes to compare [esp+12]
	; Return:
	; - < 0 if s1 < s2
	; - = 0 if s1 == s2
	; - > 0 if s1 > s2

	global  memcmp
	section .text

memcmp:
	push ebp
	mov  ebp, esp
	push esi
	push edi

	mov esi, [ebp+8]; s1
	mov edi, [ebp+12]; s2
	mov ecx, [ebp+16]; n

	;    Check if count is zero
	test ecx, ecx
	jz   .equal; If count is zero, buffers are equal

	;   Check if we can do dword comparison
	cmp ecx, 4
	jb  .byte_compare; Less than 4 bytes, use byte comparison

	;    Check alignment
	mov  eax, esi
	or   eax, edi
	test al, 3
	jnz  .byte_compare; If either pointer is not aligned, use byte comparison

	;    Compare dwords first
	mov  edx, ecx
	shr  ecx, 2; Convert count to dwords
	repe cmpsd; Compare dwords, exit on inequality or when count is zero
	jne  .compare_last_dword

	;   Handle remaining bytes
	mov ecx, edx
	and ecx, 3; Get remainder bytes (0-3)
	jz  .equal; If no remainder and all dwords were equal, buffers are equal

.byte_compare:
	;    Byte-by-byte comparison
	repe cmpsb; Compare bytes, exit on inequality or when count is zero
	jz   .equal; If we exited due to count reaching zero, buffers are equal

	;     Return difference between last compared bytes
	movzx eax, byte [esi-1]
	movzx edx, byte [edi-1]
	sub   eax, edx
	jmp   .done

.compare_last_dword:
	;   We found unequal dwords, need to find which byte differs
	sub esi, 4; Go back to beginning of unequal dword
	sub edi, 4

	;     Compare first byte
	movzx eax, byte [esi]
	movzx edx, byte [edi]
	cmp   eax, edx
	jne   .byte_diff

	;     Compare second byte
	movzx eax, byte [esi+1]
	movzx edx, byte [edi+1]
	cmp   eax, edx
	jne   .byte_diff

	;     Compare third byte
	movzx eax, byte [esi+2]
	movzx edx, byte [edi+2]
	cmp   eax, edx
	jne   .byte_diff

	;     Must be the fourth byte
	movzx eax, byte [esi+3]
	movzx edx, byte [edi+3]

.byte_diff:
	sub eax, edx
	jmp .done

.equal:
	xor eax, eax; Return 0 (buffers are equal)

.done:
	pop edi
	pop esi
	pop ebp
	ret
