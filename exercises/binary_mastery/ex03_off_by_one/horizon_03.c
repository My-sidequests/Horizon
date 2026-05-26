/*
** horizon_03.c — Off-by-One
**
** Goal    : exploit a single-byte overflow to redirect execution.
** Method  : corrupt the LSB of the Saved RBP to pivot the stack frame.
**
** Compile : gcc -fno-stack-protector -no-pie -o horizon_03 horizon_03.c
*/

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

/* Never reached in normal flow. */
void secret_checkpoint(void)
{
    printf("\n[SUCCESS] Off-by-one mastered. Stack pivoted.\n");
    exit(0);
}

/*
** Vulnerability: the loop runs i <= 16 instead of i < 16.
** It writes exactly 17 bytes into a 16-byte buffer,
** overflowing by exactly one byte into the Saved RBP.
*/
void do_copy(char *source)
{
    char buffer[16];
    int  i;

    for (i = 0; i <= 16; i++)
        buffer[i] = source[i];
}

int main(void)
{
    char input[128];

    printf("horizon-03 > ");
    fflush(stdout);
    read(0, input, 127);
    do_copy(input);
    return (0);
}
