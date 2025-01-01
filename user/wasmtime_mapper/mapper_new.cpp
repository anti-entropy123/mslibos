#include <sys/time.h>
#include <stdio.h>
#include <iostream>
#include <vector>
#include <cstring>
#include <stdlib.h>
#include <cctype>
#include <ctype.h>
#include <fstream>
#include <sstream>
#include <algorithm>
#include <unordered_map>
#include <string.h>
#include <unistd.h>

using namespace std;

#define MAX_WORD_LENGTH 50
#define MAX_SLOT_NUM 10
#define MAX_WORDS 18000000
#define MAX_BUFFER_SIZE 500000

string words[MAX_WORDS];
int counts[MAX_WORDS];

__attribute__((import_module("env"), import_name("buffer_register"))) void buffer_register(void *slot_name, int name_size, void *buffer, int buffer_size);

void get_time() {
    timeval tv{};
    gettimeofday(&tv, nullptr);
    printf("%lld.%06lld\n", tv.tv_sec, tv.tv_usec);
}

long long get_time2() {
    timeval tv{};
    gettimeofday(&tv, nullptr);
    // printf("%lld.%06lld\n", (long long)tv.tv_sec, tv.tv_usec);
    return (long long)tv.tv_sec * 1000000 + tv.tv_usec;
}

void to_lowercase(std::string& s) {
    for (char& c : s) {
        c = std::tolower(c);
    }
}

char nc(FILE *stream) {
  static char buf[1<<27], *p1 = buf, *p2 = buf;
  return p1 == p2 && (p2 = (p1 = buf) + fread(buf, 1, 1 << 27, stream), p1 == p2) ? EOF : *p1 ++;
}

char* readfile(FILE *stream) {
  int cnt = 0;
  char *buffer = (char *)malloc(MAX_WORD_LENGTH * sizeof(char));
  while (buffer[cnt] = nc(stream)) {
    if (buffer[cnt] == ' ' || buffer[cnt] == '\n' || buffer[cnt] == EOF) {
        buffer[cnt] = 0;
        break;
    }
    cnt++;
  }
  return cnt > 0 ? buffer : 0;
}

int main(int argc, char* argv[])  {
    int id = atoi(argv[1]);
    int reducer_num = atoi(argv[2]);
    cout << "mapper_" << id << " start!" << endl;

    string word;
    unordered_map<string, int> word_map;

    string input_file = "fake_data_" + to_string(id) + ".txt";
    // c 快读
    // FILE *file = fopen(input_file.c_str(), "r");
    // printf("start reading file\n");
    // get_time();
    // while(true) {
    //     word = readfile(file);
    //     if (word.empty()) {
    //         break; // 假设空字符串表示没有更多的单词可以读取
    //     }
    //     to_lowercase(word);
    //     ++word_map[word];
    // }
    // printf("finish reading file\n");
    // get_time();

    // 一次性读
    printf("start reading file\n");
    get_time();
    ifstream file(input_file);
    string content((std::istreambuf_iterator<char>(file)), istreambuf_iterator<char>());
    printf("finish reading file\n");
    get_time();
    printf("start com\n");
    get_time();
    transform(content.begin(), content.end(), content.begin(),
                [](unsigned char c) { return std::tolower(c); });
    // 使用 istringstream 分割单词
    istringstream sfile(content);
    while (sfile >> word) {
        ++word_map[word];
    }
    file.close();

    // 逐个word读
    // printf("start reading file\n");
    // get_time();
    // ifstream file(input_file);
    // while (file >> word) {

    // }
    // file.close();
    // printf("finish reading file\n");
    // get_time();
    // printf("start com\n");
    // get_time();
    // ifstream file2(input_file);
    // while (file2 >> word) {
    //     transform(word.begin(), word.end(), word.begin(),
    //                [](unsigned char c) { return std::tolower(c); });
    //     ++word_map[word];
    // }
    // file2.close();
    
    cout << "mapper_" << id << " read success!" << endl;
    cout << word_map.size() << endl;

    string buffer[MAX_SLOT_NUM];
    string slot_name[MAX_SLOT_NUM];

    for (auto& pair : word_map) {
        int partition_index = hash<string>{}(pair.first) % reducer_num;
        slot_name[partition_index] = "buffer_" + to_string(partition_index) + "_" + to_string(id);
        buffer[partition_index].append(pair.first + " " + to_string(pair.second) + "\n");
    }

    for (int i = 0; i < reducer_num; i++) {
        buffer[i].resize(MAX_BUFFER_SIZE, '\0');
    }
    
    int word_index = word_map.size();
    cout << "mapper_" << id << " solved " << word_index << " words!" << endl;
    
    printf("finish com\n");
    get_time();
    for (int i = 0; i < reducer_num; i++) {
        buffer_register((void*)slot_name[i].c_str(), slot_name[i].length(), (void*)buffer[i].c_str(), buffer[i].size());
        // cout << "buffersize" << i << ": " << buffer[i].size() << endl;
    }
    cout << "mapper_" << id << " write finished!" << endl;
    printf("finish data_trans\n");
    get_time();
    return 0;
}