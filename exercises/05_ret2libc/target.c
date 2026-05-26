/*
** target.c — Return-to-libc (NX Bypass)
**
** Compile: gcc -fno-stack-protector -no-pie -o target target.c
**
** system() is called once to ensure it is linked into the binary
** and its symbol is easy to locate with nm/gdb.
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void setup(void)
{
    system("true");
}

void vulnerable(void)
{
    char buffer[16];
    read(0, buffer, 256);
}

int main(void)
{
    setup();
    printf("target-05 > ");
    fflush(stdout);
    vulnerable();
    return (0);
}
