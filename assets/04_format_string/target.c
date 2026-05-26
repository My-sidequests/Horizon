/*
** Horizon — Exercise 04: Format String Attack
** Compile: gcc -no-pie -o target target.c
*/
#include <stdio.h>
#include <unistd.h>

int secret = 0x4242;

int main(void)
{
    char buffer[64];
    int  n;

    printf("horizon-04 > ");
    fflush(stdout);

    n = read(0, buffer, 63);
    if (n > 0)
        buffer[n - 1] = '\0';

    /* Vulnerability: user input used directly as format string. */
    printf(buffer);
    printf("\n");

    if (secret == 0x1337)
        printf("[SUCCESS] Memory is no longer a mystery.\n");
    else
        printf("secret = 0x%x — not quite.\n", secret);

    return (0);
}
