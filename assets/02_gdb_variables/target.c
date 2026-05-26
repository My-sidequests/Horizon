/*
** Horizon — Exercise 02: GDB Variables
** Compile: gcc -g -o target target.c
*/
#include <stdio.h>

int main(void)
{
    int key = 0;

    printf("Key is: %d\n", key);

    if (key == 0x1337)
    {
        printf("[SUCCESS] You changed the key.\n");
        printf("Flag: HRZ{memory_bends}\n");
    }
    else
        printf("Key must be 0x1337 (decimal: %d). Try harder.\n", 0x1337);

    return (0);
}
