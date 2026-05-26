/*
** Horizon — Exercise 11: Return-to-libc (NX Bypass)
** Compile: gcc -fno-stack-protector -no-pie -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void setup(void) { system("true"); }

void vulnerable(void)
{
    char buffer[16];
    read(0, buffer, 256);
}

int main(void)
{
    setup();
    printf("horizon-11 > ");
    fflush(stdout);
    vulnerable();
    return (0);
}
