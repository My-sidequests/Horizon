/*
** Horizon — Exercise 05: Return-to-libc (NX Bypass)
** Compile: gcc -fno-stack-protector -no-pie -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

/* Ensures system() is linked and its symbol is easily found. */
void setup(void) { system("true"); }

void vulnerable(void)
{
    char buffer[16];
    read(0, buffer, 256);
}

int main(void)
{
    setup();
    printf("horizon-05 > ");
    fflush(stdout);
    vulnerable();
    return (0);
}
