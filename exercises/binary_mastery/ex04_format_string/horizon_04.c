/*
** horizon_04.c — Format String Attack
**
** Goal    : overwrite secret_key with 0x1337 using a format string bug.
**
** Compile : gcc -no-pie -o horizon_04 horizon_04.c
*/

#include <stdio.h>
#include <string.h>
#include <unistd.h>

/* Hidden global variable in the BSS segment. */
int secret_key = 0x4242;

int main(void)
{
    char buffer[64];
    int  n;

    printf("horizon-04 > ");
    fflush(stdout);

    n = read(0, buffer, 63);
    if (n > 0)
        buffer[n - 1] = '\0'; /* strip newline */

    /*
    ** Vulnerability: user input passed directly as format string.
    ** Any %p, %x, %n in the input is interpreted by printf.
    */
    printf(buffer);
    printf("\n");

    if (secret_key == 0x1337)
        printf("[SUCCESS] Memory is no longer a mystery to you.\n");
    else
        printf("Current key: 0x%x — not quite.\n", secret_key);

    return (0);
}
