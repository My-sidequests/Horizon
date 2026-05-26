/*
** Horizon — Exercise 05: Valgrind — Use-After-Free
** Compile: gcc -g -o target target.c
*/
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct  s_node
{
    int         value;
    char        label[32];
}               t_node;

int main(void)
{
    t_node *node = malloc(sizeof(t_node));

    node->value = 42;
    strcpy(node->label, "HRZ{dangling_ptr}");

    printf("Node value: %d\n", node->value);

    free(node);

    /* Bug: accessing node after free.
    ** Valgrind catches this. GDB: break before free → print node->label
    ** Or after free: the memory is not zeroed — label is still readable. */
    printf("Value after free: %d\n", node->value);
    node->value = 99;

    return (0);
}
