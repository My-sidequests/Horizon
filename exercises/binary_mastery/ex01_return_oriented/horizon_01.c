/*
** horizon_01.c — Return-Oriented Hijacking
**
** Goal    : make win() execute without calling it directly.
** Method  : overflow the buffer via stdin to overwrite the return address.
**
** Compile : gcc -fno-stack-protector -no-pie -o horizon_01 horizon_01.c
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

/* This function is never called in the normal control flow. */
void win(void)
{
    printf("\n[SUCCESS] You hijacked the return address!\n");
    exit(0);
}

/*
** Vulnerability: read() accepts 128 bytes into a 16-byte buffer.
** The overflow overwrites the saved RBP, then the return address.
*/
void vulnerable(void)
{
    char buffer[16];
    read(0, buffer, 128);
}

int main(void)
{
    printf("horizon-01 > ");
    fflush(stdout);
    vulnerable();
    printf("Normal return. Nothing to see here.\n");
    return (0);
}
