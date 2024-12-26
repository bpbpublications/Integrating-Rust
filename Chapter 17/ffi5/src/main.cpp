// File: main.cpp

#include <iostream>

extern "C" {
    struct MyStruct {
        int x;
        int y;
    };

    int process_struct(MyStruct input);
}

int main() {
    MyStruct input{3, 5};

    int result = process_struct(input);

    std::cout << "Result of processing struct: " << result << std::endl;

    return 0;
}
