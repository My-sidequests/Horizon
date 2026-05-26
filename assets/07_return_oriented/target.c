/*
** Horizon — Exercise 07: Return-Oriented Hijacking
** Compile: gcc -fno-stack-protector -no-pie -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void win(void)
{
    printf("\n[SUCCESS] You hijacked the return address!\n");
    printf("Flag: HRZ{ret_hijacked}\n");
    exit(0);
}

void vulnerable(void)
{
    char buffer[16];
    read(0, buffer, 128);
}

int main(void)
{
    printf("horizon-07 > ");
    fflush(stdout);
    vulnerable();
    printf("Normal return. Nothing happened.\n");
    return (0);
}
