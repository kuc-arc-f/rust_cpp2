// sample.cpp
#include <iostream>
#include <cstring>
#include <string>
#include "include/http_client.cpp"

extern "C" {

    __declspec(dllexport) int test_http() {
        std::string url = "http://localhost:8080/";

        HttpClient client(30 /*timeout*/, true /*verify_ssl*/);
        auto resp = client.get(url);
        print_response("GET:", resp);
        
        return 1;
    }
    // 整数の加算（既存）
    __declspec(dllexport) int add(int a, int b) {
        return a + b;
    }

    // メモリ解放用（必要に応じて）
    __declspec(dllexport) void free_string(char* ptr) {
        delete[] ptr;
    }
}