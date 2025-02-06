section .data
test_header     db "GDT Memory Test Results (32-bit):", 0
test1_msg       db "Test 1 (Start of segment): ", 0
test2_msg       db "Test 2 (Video memory): ", 0
test3_msg       db "Test 3 (Safe memory region): ", 0
success_msg     db "Success", 0
failure_msg     db "Failed", 0
error_msg       db "Test failed - System halted", 0
ok      db "Tests succesfull - OK", 0

section .text
global  test_memory_access

test_memory_access:
	;    Set up stack frame and preserve registers
	push ebp
	mov  ebp, esp
	push ebx
	push esi
	push edi

	mov  esi, test_header
	mov  ebx, 0xB8000; VGA Magic Number
	call print_string_line

	;    Test 1: Beginning of segment (0x00000000)
	mov  ebx, 0xB8000 + 160; Second line
	mov  esi, test1_msg
	call print_string_line

	;   Test low memory access
	mov byte [0x00000000], 0x55
	mov al, [0x00000000]
	cmp al, 0x55
	jne .test1_failed
	mov esi, success_msg
	jmp .print_test1_result

.print_test1_result:
	call print_string_line

	;    Test 2: Video memory access (0xB8000)
	mov  ebx, 0xB8000 + 320; Third line
	mov  esi, test2_msg
	call print_string_line

	;   Test video memory access
	mov byte [0xB8000], 'X'
	mov al, [0xB8000]
	cmp al, 'X'
	jne .test2_failed
	mov esi, success_msg
	jmp .print_test2_result

.print_test2_result:
	call print_string_line

	;    Test 3: Safe memory region (1MB mark)
	mov  ebx, 0xB8000 + 480; Fourth line
	mov  esi, test3_msg
	call print_string_line

	;   Test access at 1MB boundary (a more reasonable test point for 32-bit)
	mov byte [0x100000], 0xAA; 1MB mark
	mov al, [0x100000]
	cmp al, 0xAA
	jne .test3_failed
	mov esi, success_msg
	jmp .print_test3_result

.test1_failed:
.test2_failed:
	mov  esi, failure_msg
	call print_string_line
	jmp  halt_system

.test3_failed:
	mov esi, failure_msg
	jmp halt_system

.print_test3_result:
	call print_string_line

	mov  ebx, 0xB8000 + 800; Sixth line
	mov  esi, ok
	call print_string_line

	;   Clean up and return
	pop edi
	pop esi
	pop ebx
	pop ebp
	ret

print_string_line:
	push eax

.loop:
	lodsb ; Load next character into AL
	test  al, al; Check for null terminator
	jz    .done
	mov   byte [ebx], al; Write character
	mov   byte [ebx+1], 0x0F; White text on black background
	add   ebx, 2; Next character position
	jmp   .loop

.done:
	pop eax

	ret

halt_system:
	;    System never starts & never calls kernel_main()
	mov  ebx, 0xB8000 + 800
	mov  esi, error_msg
	call print_string_line

	cli

.halt_loop:
	hlt
	jmp .halt_loop
