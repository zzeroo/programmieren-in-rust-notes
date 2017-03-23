magic:
    push    rbp       ; push old %rbp on stack
                      ;   old %rbp is pushed onto the stack to be stored until the end of the function, %rbp ist der 'base pointer'
    mov     rbp, rsp  ; new %rsp is where %rsp points now
                      ;   verschiebe %rsp 'stack pointer' nach %rbp
                      ;   then, the %rbp 'base pointer' is set to the value of the new %rsp, http://stackoverflow.com/questions/16088946/understanding-c-disassembled-call
    mov     rcx, rdi  ; 1. Argument speichern
                      ;   %rdi ist der Abfang der User Daten; wird in Register C gespeichert
    mov     esi, 2    ; schreibe 2 nach %esi
.LBB0_1:              ; loop {
    mov     al, 1     ; schreibe 1 nach in die ersten Bytes des A Registers, %al Low byte of the ax register https://www.quora.com/What-is-al-in-a-mov-instruction-in-Assembly
    cmp     rsi, rcx  ; 2. Argument %rsi
                      ;   vergleiche %rsi dem Wert in Register C %rcx (4. Argument)
    jae     .LBB0_4   ; %jae means jump if above or equal. It will jump if the `carry` flag is equal to 0. Wenn kein carry => Spring zum ENDE http://stackoverflow.com/questions/25575849/x86-jae-instruction
    xor     edx, edx  ; %edx leeren
    mov     rax, rcx  ; register C nach Register A schreiben
    div     rsi
    inc     rsi       ; +1
    test    rdx, rdx  ; Register D ; do while()
    jne     .LBB0_1   ; loop
    xor     eax, eax  ; %exa leeren
.LBB0_4:              ; ENDE
    pop     rbp       ; restore %rbp 'base pointer'
    ret               ; return
                      ;   sprint zur Ruecksprungsaddresse
