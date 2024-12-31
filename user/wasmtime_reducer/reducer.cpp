#include <sys/time.h>
#include <iostream>
#include <vector>
#include <cstring>
#include <stdlib.h>
#include <cctype>
#include <fstream>
#include <sstream>
using namespace std;

#define MAX_SLOT_NUM 10
#define MAX_WORDS 20000000
#define MAX_BUFFER_SIZE 500000

__attribute__((import_module("env"), import_name("access_buffer"))) void access_buffer(void *slot_name, int name_size, void *buffer, int buffer_size);

void get_time() {
    timeval tv{};
    gettimeofday(&tv, nullptr);
    
    printf("%lld.%06ld\n", (long long)tv.tv_sec, tv.tv_usec);
}

string words[MAX_WORDS];
int counts[MAX_WORDS];

int main(int argc, char* argv[]) {
    int id = atoi(argv[1]);
    int mapper_num = atoi(argv[2]);
    cout << "reducer_" << id << " start!" << endl;
    int slot_num = mapper_num;
    string slot_name[MAX_SLOT_NUM];
    string buffer[MAX_SLOT_NUM];

    for (int i = 0; i < slot_num; ++i) {
        slot_name[i] = "buffer_" + to_string(i) + "_" + to_string(id);
        // size_t bufferSize = faasmReadStateSize(slot_name[i].c_str());
        buffer[i] = string(MAX_BUFFER_SIZE, 0);
        // faasmReadState(slot_name[i].c_str(), (uint8_t*)buffer[i].c_str(), bufferSize);
        access_buffer((void*)slot_name[i].c_str(), slot_name[i].length(), (void*)buffer[i].c_str(), buffer[i].size());
    }

    // vector<string> words;
    // vector<int> count;
    int word_index = 0;

    for (int i = 0; i < slot_num; i++) {
        istringstream iss(buffer[i]);
        string word;
        int num;
        while (iss >> word >> num) {
            int found = 0;
            for (int i = 0; i < word_index; i++) {
                if (words[i] == word) {
                    counts[i] += num;
                    found = 1;
                    break;
                }
            }
            if (!found) {
                // words.push_back(word);
                // count.push_back(num);
                words[word_index] = word;
                counts[word_index] = num;
                word_index++;
            }
        }
    }

    cout << "reducer_" << id << " solved " << word_index << " words!" << endl;

    // string output_file = "faasm://reducer_" + to_string(id) + "_" + to_string(get_time()) + ".txt";
    // ofstream output(output_file);
    // for (int i = 0; i < word_index; i++) {
    //     output << words[i] << " " << count[i] << endl;
    // }
    // output.close();
    
    cout << "reducer_" << id << " finished!" << endl;
    get_time();
    return 0;
}