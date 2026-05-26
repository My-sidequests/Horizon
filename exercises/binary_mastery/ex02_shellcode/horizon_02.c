/*
** horizon_02.c — Shellcode Injection
**
** Goal    : inject and execute your own machine code.
**           No target function exists — you write the code to run.
**
** Compile : gcc -fno-stack-protector -z execstack -no-pie -o horizon_02 horizon_02.c
*/

#include <stdio.h>
#include <unistd.h>

/*
** Vulnerability: read() accepts 256 bytes into a 64-byte buffer.
** With -z execstack, memory on the stack is executable:
** shellcode injected into buffer can be jumped to directly.
*/
void vulnerable(void)
{
    char buffer[64];
    read(0, buffer, 256);
}

int main(void)
{
    printf("horizon-02 > ");
    fflush(stdout);
    vulnerable();
    return (0);
}
