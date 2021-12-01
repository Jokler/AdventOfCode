; Advent of Code Day 1 Part 1
; This kinda sucks currently

%include "src/unistd_64.inc"

section .text
global _start

_start:
    ; open file
    mov rax, sys_open
    mov rdi, input_path
    mov rsi, open_flags
    mov rdx, 0 ; no mode for now
    syscall

    ; error handling
    cmp rax, 0
    jnl input_opened

    xor rbx, rbx
    sub rbx, rax

    mov rax, sys_exit
    mov rdi, rbx
    syscall

input_opened:
    mov [fd_input], rax

    ; doesnt work in the loop for some reason
    mov rdi, [fd_input]

    mov r10, -1 ; result, first value will always be an increase from 0(r14)
    xor r14, r14 ; previous line
read_loop:
    mov rax, sys_read
    mov rsi, buf
    mov rdx, 10240                  ; TODO the read_loop does not actually work correctly
    syscall                         ;      so for now I read the entire file instead

    ; error handling
    cmp rax, 0
    push rax
    jl read_error
    pop rax

    mov r13, rax                    ; r13: bytes in buf

    cmp r13, 0                      ; if bytes read == 0
    je read_line_break
    xor r12, r12                    ; r12: result
    xor rbx, rbx                    ; rbx: number bytes read
read_number_loop:
    cmp byte [buf+rbx], 0xA         ; if current == '\n'
    jne continue_number
    cmp r12, r14                    ; if r12 > previous line
    mov r14, r12
    jng no_increase
    add r10, 1                      ; result += 1
no_increase:
    xor r12, r12
    jmp read_line_continue

continue_number:
    ; error handling
    cmp byte [buf+rbx], '0'
    jl bad_number_error
    cmp byte [buf+rbx], '9'
    jg bad_number_error

    xor rdx, rdx
    mov dl, [buf+rbx]
    sub dl, '0'

    imul r12, 10
    add r12, rdx

read_line_continue:
    add rbx, 1
    cmp r13, rbx
    jne read_number_loop
    je read_loop

read_line_break:
    ; DONE
    mov rax, sys_close
    mov rdi, [fd_input]
    syscall

    ; error handling
    cmp rax, 0
    push rax
    jl close_error
    pop rax

    mov rax, sys_write
    mov rdi, stdout
    mov rsi, success_msg
    mov rdx, success_msg_len
    syscall

exit_success:
    mov rax, r10
    xor r12, r12

    sub rsp, 1
    mov byte [rsp+1], 0xA
    add r12, 1

output_loop:
    xor rcx, rcx
    xor rdx, rdx
    mov rcx, 10
    div rcx
    add rdx, '0'
    sub rsp, 1
    mov [rsp+1], dl
    add r12, 1

    cmp rax, 0
    jne output_loop

    add rsp, 1
    mov rax, sys_write
    mov rdi, stdout
    mov rsi, rsp
    mov rdx, r12
    syscall
    sub rsp, 1

    add rsp, r12

    mov rax, sys_exit
    mov rdi, 0
    syscall

bad_number_error:
    mov rax, sys_write
    mov rdi, stdout
    mov rsi, bad_number_error_msg
    mov rdx, bad_number_error_msg_len
    syscall

    mov rax, sys_exit
    mov rdi, [buf+rbx]      ; return non number char
    syscall

read_error:
    mov rax, sys_write
    mov rdi, stdout
    mov rsi, read_error_msg
    mov rdx, read_error_msg_len
    syscall

    pop rax                 ; error

    xor rdi, rdi
    sub rdi, rax
    mov rax, sys_exit
    syscall

close_error:
    mov rax, sys_write
    mov rdi, stdout
    mov rsi, close_error_msg
    mov rdx, close_error_msg_len
    syscall

    pop rax     ; error

    xor rdi, rdi
    sub rdi, rax
    mov rax, sys_exit
    syscall

section .data
    stdout equ 1
    open_flags equ 0
    line_feed equ 0xA
    input_path db "input.txt", 0
    success_msg db "Success!", 0xA
    ; $ = current location in memory
    success_msg_len equ $-success_msg

    bad_number_error_msg db "Not a number!", 0xA
    bad_number_error_msg_len equ $-bad_number_error_msg

    read_error_msg db "Failed to read file!", 0xA
    read_error_msg_len equ $-read_error_msg

    close_error_msg db "Failed to close file!", 0xA
    close_error_msg_len equ $-close_error_msg

section .bss
    fd_input resq 1     ; file descriptor
    buf resb 10240      ; Buffer for reading
