global start, error
extern check_multiboot, check_cpuid, check_long_mode
extern setup_page_tables, enable_paging
extern stack_top, gdt64.pointer, gdt64.code_segment
extern long_mode_start

section .text
bits 32

start:
	mov esp, stack_top

	; checks
	call check_multiboot
	call check_cpuid
	call check_long_mode

	; setups
	call setup_page_tables
	call enable_paging

	lgdt [gdt64.pointer]
	jmp gdt64.code_segment: long_mode_start
	jmp halt

error:
	; print 'ERR: <err>'
	mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov byte  [0xb800a], al
	jmp halt

halt:
	; halt the CPU
	mov word [0xb8f00], 0x0f5a
	mov word [0xb8f02], 0x0f5a
	mov word [0xb8f04], 0x0f5a
	hlt