/*
** horizon_05.c — Return-to-libc (NX Bypass)
**
** Goal    : call system() from libc without injecting shellcode.
**           NX is active — the stack is NOT executable.
**
** Compile : gcc -fno-stack-protector -no-pie -o horizon_05 horizon_05.c
**
** Note    : system() is called once explicitly to ensure it is linked
**           into the binary and easy to find with nm/gdb.
*/

#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <unistd.h>

/* Forces the linker to include system() in the symbol table. */
void setup(void)
{
    system("true");
}

/*
** Vulnerability: read() accepts 256 bytes into a 16-byte buffer.
** Classic overflow — but you cannot execute code on the stack.
*/
void do_overflow(void)
{
    char buffer[16];
    read(0, buffer, 256);
}

int main(void)
{
    setup();
    printf("horizon-05 > ");
    fflush(stdout);
    do_overflow();
    return (0);
}
