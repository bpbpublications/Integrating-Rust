// File: main.c

#include <stdio.h>

// Declaration of the Rust function
extern int add(int a, int b);

int main() {
    int result = add(5, 6);
    printf("Result of adding numbers: %d\n", result);
    return 0;
}
