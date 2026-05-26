/*
** Horizon — Exercise 04: Valgrind — Memory Leak
** Compile: gcc -g -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char    *create_message(const char *text)
{
    char *msg = malloc(strlen(text) + 1);
    strcpy(msg, text);
    return (msg);
}

int main(void)
{
    char *a    = create_message("hello");
    char *b    = create_message("world");
    char *c    = create_message("horizon");
    char *flag = create_message("HRZ{leak_found}");

    printf("%s %s %s\n", a, b, c);

    free(a);
    /* Bug: b, c, and flag are never freed.
    ** Valgrind will report 3 leaked allocations.
    ** Run in GDB, break after this line, then: print flag */
    return (0);
}
