/*
** Horizon — Exercise 06: Valgrind — Invalid Read
** Compile: gcc -g -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>

int sum_array(int *arr, int size)
{
    static const char flag[] = "HRZ{one_too_many}";
    int total = 0;
    int i     = 0;

    /* Bug: loop runs one past the end (i <= size instead of i < size). */
    while (i <= size)
        total += arr[i++];

    /* flag is never printed.
    ** Run valgrind to catch the overread, then GDB:
    ** break sum_array → print flag */
    (void)flag;
    return (total);
}

int main(void)
{
    int *arr = malloc(5 * sizeof(int));

    arr[0] = 10; arr[1] = 20; arr[2] = 30; arr[3] = 40; arr[4] = 50;

    printf("Sum: %d\n", sum_array(arr, 5));

    free(arr);
    return (0);
}
