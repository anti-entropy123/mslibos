#include <sys/time.h>
#include <stdio.h>
#include <iostream>
#include <vector>
#include <cstring>
#include <stdlib.h>
#include <cctype>
#include <fstream>
#include <algorithm>

using namespace std;

#define MAX_SLOT_NUM 10
#define MAX_WORDS 20000000
#define MAX_BUFFER_SIZE 500000

string words[MAX_WORDS];
int counts[MAX_WORDS];

__attribute__((import_module("env"), import_name("buffer_register"))) void buffer_register(void *slot_name, int name_size, void *buffer, int buffer_size);

void get_time() {
    timeval tv{};
    gettimeofday(&tv, nullptr);
    printf("%lld.%lld\n", tv.tv_sec, tv.tv_usec);
}

int main(int argc, char* argv[])  {
    int id = atoi(argv[1]);
    int reducer_num = atoi(argv[2]);
    cout << "mapper_" << id << " start!" << endl;

    string input_file = "fake_data_" + to_string(id) + ".txt";
    ifstream file(input_file);
    
    int word_index = 0;

    string word;
    // vector<string> words;
    // vector<int> counts;
    

    printf("start reading file\n");
    get_time();
    while (file >> word) {
        transform(word.begin(), word.end(), word.begin(),
                   [](unsigned char c) { return std::tolower(c); });
        int found = 0;
        for (int i = 0; i < word_index; i++) {
            if (words[i] == word) {
                counts[i]++;
                found = 1;
                break;
            }
        }
        if (!found) {
            words[word_index] = word;
            counts[word_index] = 1;
            // words.push_back(word);
            // counts.push_back(1);
            // words.emplace_back(word);
            // count.emplace_back(1);
            word_index++;
        }
    }
    printf("finish reading file\n");
    get_time();
    // return 0;
    file.close();

    cout << "mapper_" << id << " read success!" << endl;

    string buffer[MAX_SLOT_NUM];
    string slot_name[MAX_SLOT_NUM];
    int slot_index = 0;

    for (int i = 0; i < word_index; i++) {
        int partition_index = i % reducer_num;
        string slot = "buffer_" + to_string(partition_index) + "_" + to_string(id);
        int found = 0;
        for (int j = 0; j < slot_index; j++) {
            if (slot_name[j] == slot) {
                found = 1;
                buffer[j].append(words[i] + ": " + to_string(counts[i]) + "\n");
                break;
            }
        }
        
        if (!found) {
            slot_name[slot_index] = slot;
            buffer[slot_index] = words[i] + ": " + to_string(counts[i]) + "\n";
            slot_index++;
        }
    }

    for (int i = 0; i < MAX_SLOT_NUM; i++) {
        buffer[i].resize(MAX_BUFFER_SIZE, '\0');
    }
    
    
    cout << "mapper_" << id << " solved " << word_index << " words!" << endl;
    for (int i = 0; i < slot_index; i++) {
        buffer_register((void*)slot_name[i].c_str(), slot_name[i].length(), (void*)buffer[i].c_str(), buffer[i].size());
        // cout << "buffersize" << i << ": " << buffer[i].size() << endl;
    }
    cout << "mapper_" << id << " write finished!" << endl;
    
    return 0;
}