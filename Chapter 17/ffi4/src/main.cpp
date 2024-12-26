// File: main.cpp

#include <iostream>
#include <cstring>

extern "C" {
    char* process_string(const char* input);
}

int main() {
    const char* input = "Hello, Rust!";

    char* processed_string = process_string(input);

    std::cout << "Original string: " << input << std::endl;
    std::cout << "Processed string: " << processed_string << std::endl;

    return 0;
}
