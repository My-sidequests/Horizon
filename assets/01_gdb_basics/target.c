/*
** Horizon — Exercise 01: GDB Basics
** Compile: gcc -g -o target target.c
*/
#include <stdio.h>

void success(void)
{
    printf("[SUCCESS] You called a hidden function.\n");
    printf("Flag: HRZ{call_is_power}\n");
}

void decoy_a(void) { printf("Nope.\n"); }
void decoy_b(void) { printf("Not here either.\n"); }
void decoy_c(void) { printf("Keep looking.\n"); }

int main(void)
{
    decoy_a();
    decoy_b();
    decoy_c();
    printf("Nothing useful happened.\n");
    return (0);
}
