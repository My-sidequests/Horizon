/*
** Horizon — Exercise 03: GDB Crash Analysis
** Compile: gcc -g -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>

void process(int *data)
{
    static const char flag[] = "HRZ{null_speaks}";

    /* Bug: data is NULL when count is zero — dereferences a null pointer. */
    printf("First value: %d\n", *data);

    /* flag is never printed in normal flow.
    ** Crash here in GDB, then: frame 0 → print flag */
    (void)flag;
}

void load_data(int count)
{
    int *data = NULL;

    if (count > 0)
        data = malloc(count * sizeof(int));
    process(data);
    free(data);
}

int main(void)
{
    load_data(0);
    return (0);
}
