// sample.cpp
#include <iostream>
#include <cstring>
#include <string>

extern "C" {

    // 整数の加算（既存）
    __declspec(dllexport) int add(int a, int b) {
        return a + b;
    }

    // Rustから文字列を受け取り、処理して返す
    // message: Rustから送られてくる文字列
    // output_buffer: 戻り値を格納するバッファ（Rust側で用意）
    // buffer_size: バッファのサイズ
    __declspec(dllexport) void process_string(
        const char* message,      // 入力文字列
        char* output_buffer,      // 出力バッファ
        int buffer_size           // バッファサイズ
    ) {
        // 受信した文字列を表示
        //std::cout << "C++が受信: " << message << std::endl;
        std::cout << "C++ Receive:" << message << std::endl;
        
        // 処理：受信文字列に " [C++から処理済み]" を追加
        //std::string result = std::string(message) + " [C++から処理済み]";
        std::string result = std::string(message) + " [C++ from receive]";
        
        // バッファサイズをチェックしてコピー
        if (result.length() < static_cast<size_t>(buffer_size)) {
            strcpy_s(output_buffer, buffer_size, result.c_str());
        } else {
            //strcpy_s(output_buffer, buffer_size, "バッファ不足");
            strcpy_s(output_buffer, buffer_size, "buffer too small");
        }
        
        //std::cout << "C++が送信: " << output_buffer << std::endl;
        std::cout << "C++ Send:" << output_buffer << std::endl;
    }

    // 文字列を大文字に変換して返す
    __declspec(dllexport) void to_uppercase(
        const char* input,        // 入力文字列
        char* output_buffer,      // 出力バッファ
        int buffer_size           // バッファサイズ
    ) {
        std::string result = std::string(input);
        
        for (char &c : result) {
            c = toupper(c);
        }
        
        if (result.length() < static_cast<size_t>(buffer_size)) {
            strcpy_s(output_buffer, buffer_size, result.c_str());
        } else {
            strcpy_s(output_buffer, buffer_size, result.c_str());
        }
        
        //std::cout << "大文字変換: " << input << " -> " << output_buffer << std::endl;
        std::cout << "Upper change: " << input << " -> " << output_buffer << std::endl;
    }

    // 文字列を連結して返す
    __declspec(dllexport) void concatenate_strings(
        const char* str1,         // 文字列1
        const char* str2,         // 文字列2
        char* output_buffer,      // 出力バッファ
        int buffer_size           // バッファサイズ
    ) {
        std::string result = std::string(str1) + std::string(str2);
        
        if (result.length() < static_cast<size_t>(buffer_size)) {
            strcpy_s(output_buffer, buffer_size, result.c_str());
        } else {
            strcpy_s(output_buffer, buffer_size, result.substr(0, buffer_size - 1).c_str());
        }
    }

    // メモリ解放用（必要に応じて）
    __declspec(dllexport) void free_string(char* ptr) {
        delete[] ptr;
    }
}