// File: main.cpp

#include <iostream>

extern "C" {
    int process_array(const int* input, size_t len);
}

int main() {
    const int input[] = {1, 2, 3, 4, 5};

    size_t len = sizeof(input) / sizeof(input[0]);

    int result = process_array(input, len);

    std::cout << "Result of processing array: " << result << std::endl;

    return 0;
}
