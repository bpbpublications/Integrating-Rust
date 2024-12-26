// File: main.c

#include <stdio.h>

// Declaration of Rust functions using the C interface
extern int add(int a, int b);
extern int subtract(int a, int b);

int main() {
    int result_add = add(5, 3);
    int result_subtract = subtract(10, 4);

    printf("Result of addition: %d\n", result_add);
    printf("Result of subtraction: %d\n", result_subtract);

    return 0;
}
