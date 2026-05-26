/*
** Horizon — Exercise 09: Off-by-One
** Compile: gcc -fno-stack-protector -no-pie -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

void secret(void)
{
    printf("\n[SUCCESS] Off-by-one mastered.\n");
    printf("Flag: HRZ{one_byte_king}\n");
    exit(0);
}

void do_copy(char *src)
{
    char buffer[16];
    int  i;

    /* Bug: <= 16 instead of < 16 */
    for (i = 0; i <= 16; i++)
        buffer[i] = src[i];
}

int main(void)
{
    char input[128];

    printf("horizon-09 > ");
    fflush(stdout);
    read(0, input, 127);
    do_copy(input);
    return (0);
}
