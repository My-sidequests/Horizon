/*
** Horizon — Exercise 08: Shellcode Injection
** Compile: gcc -fno-stack-protector -z execstack -no-pie -o target target.c
*/
#include <stdio.h>
#include <unistd.h>

void vulnerable(void)
{
    char buffer[64];
    read(0, buffer, 256);
}

int main(void)
{
    printf("horizon-08 > ");
    fflush(stdout);
    vulnerable();
    return (0);
}
