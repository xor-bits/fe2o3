global setup_page_tables, enable_paging
extern page_table_l4, page_table_l3, page_table_l2

section .text
bits 32

; page tables
setup_page_tables:
	mov eax, page_table_l3
	or  eax, 0b11 ; present, writeable
	mov [page_table_l4], eax

	mov eax, page_table_l2
	or  eax, 0b11 ; present, writeable
	mov [page_table_l3], eax

	mov ecx, 0 ; counter

.loop:
	mov eax, 0x200000 ; 2MiB
	mul ecx,
	or  eax, 0b10000011 ; present, writeable, huge page
	mov [page_table_l2 + ecx * 8], eax

	inc ecx ; inc counter
	cmp ecx, 512 ; check if the whole table is mapped
	jne .loop ; if not: continue

	ret

; paging
enable_paging:
	; pass page table location to the cpu
	mov eax, page_table_l4
	mov cr3, eax

	; enable PAE
	mov eax, cr4
	or  eax, 1 << 5
	mov cr4, eax

	; enable long mode
	mov ecx, 0xC0000080
	rdmsr
	or  eax, 1 << 8
	wrmsr

	; enable paging
	mov eax, cr0
	or  eax, 1 << 31
	mov cr0, eax

	ret