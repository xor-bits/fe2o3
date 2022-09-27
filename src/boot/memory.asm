global page_table_l4, page_table_l3, page_table_l2
global stack_top
global gdt64.pointer, gdt64.code_segment

section .bss
align 4096

; page tables
page_table_l4:
	resb 4096
page_table_l3:
	resb 4096
page_table_l2:
	resb 4096

; stack
stack_bottom:
	resb 4096 * 4
stack_top:

; global descriptor table
section .rodata
gdt64:
	dq 0 ; zero entry
.code_segment: equ $ - gdt64
	dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
	dw $ - gdt64 - 1
	dq gdt64