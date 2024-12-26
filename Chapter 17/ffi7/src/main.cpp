// File: main.cpp

#include <iostream>
#include <stdexcept>

extern "C" {
    int safe_divide(int dividend, int divisor);
}

int main() {
    try {
        int result = safe_divide(10, 2);
        std::cout << "Result of safe division: " << result << std::endl;

        // Uncomment the line below to trigger a division by zero error
        // int error_result = safe_divide(5, 0);
    } catch (const std::exception& ex) {
        std::cerr << "Exception: " << ex.what() << std::endl;
    }

    return 0;
}
