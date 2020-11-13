#ifdef __cplusplus
extern "C"{
#endif

int func_in_rust(int input);

#ifdef __cplusplus
}
#endif

#include<iostream>

extern "C"
int func_in_cpp(int input) {
    std::cout << "in c++. calling rust..." << std::endl;
    int out = func_in_rust(input);
    return out * 3;
}