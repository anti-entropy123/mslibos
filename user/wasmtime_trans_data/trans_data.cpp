#include <sys/time.h>
#include <stdio.h>
#include <iostream>
#include <vector>
#include <cstring>
#include <stdlib.h>
#include <cctype>
#include <fstream>
#include <algorithm>
#include <unordered_map>

using namespace std;

#define MAX_BUFFER_SIZE 1024 * 64


__attribute__((import_module("env"), import_name("buffer_register"))) void buffer_register(void *slot_name, int name_size, void *buffer, int buffer_size);
__attribute__((import_module("env"), import_name("access_buffer"))) void access_buffer(void *slot_name, int name_size, void *buffer, int buffer_size);


void get_time() {
    timeval tv{};
    gettimeofday(&tv, nullptr);
    printf("%lld.%06lld\n", tv.tv_sec, tv.tv_usec);
}

int arrays[100000000];

int main()  {
    // get_time();
    // string slot_name = "tmp";
    // // string buffer(MAX_BUFFER_SIZE, '0');
    // vector<char> buffer(MAX_BUFFER_SIZE, 0);
    // buffer_register((void*)slot_name.c_str(), slot_name.length(), (void*)buffer.data(), MAX_BUFFER_SIZE);

    // get_time();
    // access_buffer((void*)slot_name.c_str(), slot_name.length(), (void*)buffer.data(), MAX_BUFFER_SIZE);
    // get_time();


    //breakdown test
    // get_time();
    // const int N = 1e9;
    // unsigned int a = 0, b = 1;
    // for (int i = 1; i <= N; i++) {
    //     a += b;
    //     b *= 2;
    // }
    // get_time();
    // printf("a: %u\n", a);

    // for (int i = 0; i < 100000000; i++) {
    //     arrays[i] = 100000000 - i;
    // }
    // get_time();
    // sort(arrays, arrays+100000000);
    // get_time();

    srand(42);

    get_time();
    for (int i = 0; i < 100000000; i++) {
        arrays[i] = rand();
    }
    get_time();
    sort(arrays, arrays + 100000000);
    get_time();
    printf("%d\n", arrays[9999999]);
    // vector<int> arrays;
    // get_time();
    // for (int i = 0; i < 100000000; i++) {
    //     arrays.push_back(1);
    // }
    // get_time();
    return 0;
}