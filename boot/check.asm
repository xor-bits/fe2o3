global check_multiboot, check_cpuid, check_long_mode
extern error

section .text
bits 32

; multiboot check
check_multiboot:
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret
.no_multiboot:
	mov al, "M"
	jmp error

; cpuid check
check_cpuid:
	pushfd
	pop eax
	mov ecx, eax
	xor eax, 1 << 21
	push eax
	popfd

	pushfd
	pop eax
	push ecx
	popfd

	cmp eax, ecx
	je .no_cpuid
	ret

.no_cpuid:
	mov al, "C"
	jmp error

; long mode check
check_long_mode:
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001
	jb .no_long_mode

	mov eax, 0x80000001
	cpuid
	test edx, 1 << 29
	jz .no_long_mode

	ret

.no_long_mode:
	mov al, "L"
	jmp error