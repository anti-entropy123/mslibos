#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#include <sys/time.h>

__attribute__((import_module("env"), import_name("buffer_register"))) void buffer_register(void *slot_name, int name_size, void *buffer, int buffer_size);

#define MAX_WORD_LENGTH 50
#define MAX_WORDS 18000000
#define MAX_SLOT_NUM 5
#define MAX_BUFFER_SIZE 500000

int count[MAX_WORDS];
char *words[MAX_WORDS];

void to_lowercase(char *str) {
    for (int i = 0; str[i]; i++) {
        str[i] = tolower(str[i]);
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

void get_now() {
    struct timeval tv;
    gettimeofday(&tv, NULL);
    printf("%lld.%lld\n", tv.tv_sec, tv.tv_usec);
}

int main(int argc, char* argv[]) {
    int id = atoi(argv[1]);
    int reducer_num = atoi(argv[2]);
    printf("mapper.c recieve: id: %d, reducer_num: %d\n", id, reducer_num);

    char input_file[30];
    sprintf(input_file, "fake_data_%d.txt", id);
    FILE *file = fopen(input_file, "r");
    if (!file) {
        perror("Failed to open input file\n");
        exit(EXIT_FAILURE);
    }

    printf("mapper_%d input file: %s\n", id, input_file);
    
    char* word;
    int word_index = 0;
    // int test_num = 0;
    get_now();
    while(word = readfile(file)) {
        to_lowercase(word);
        int found = 0;

        for (int i = 0; i < word_index; i++) {
            if (strcmp(words[i], word) == 0) {
                count[i]++;
                found = 1;
                break;
            }

        }
        
        if (!found) {
            words[word_index] = strdup(word);
            count[word_index]++;
            word_index++;
        }

    }
    get_now();

    fclose(file);
    // return 0;
    printf("mapper_%d_index: %d\n", id, word_index);


    char **slot_name = (char **)malloc(MAX_SLOT_NUM * sizeof(char*));
    for (int i = 0; i < MAX_SLOT_NUM; i++) {
        slot_name[i] = (char *)malloc(20 * sizeof(char));
    }
    // char *slot_name[MAX_SLOT_NUM];
    char **buffer = (char **)malloc(MAX_SLOT_NUM * sizeof(char*));
    for (int i = 0; i < MAX_SLOT_NUM; i++) {
        buffer[i] = (char *)malloc(MAX_BUFFER_SIZE * sizeof(char));
    }
    // char *buffer[MAX_SLOT_NUM];
    char slot[20];
    int bufferSize = MAX_BUFFER_SIZE;
    int slot_index = 0;
    
    char *ptr[MAX_SLOT_NUM];
    for (int i = 0; i < reducer_num; i++) {
        ptr[i] = buffer[i];
    }
    
    for (int i = 0; i < word_index; i++) {
        int partition_index = i % reducer_num;
        sprintf(slot, "buffer_%d_%d", partition_index, id);
        int found = 0;
        for (int j = 0; j < slot_index; j++) {
            if (strcmp(slot_name[j], slot) == 0) {
                found = 1;
                // sprintf(buffer[j] + total_len[j], "%s: %d\n", words[i], count[i]);
                char temp[100];
                snprintf(temp, sizeof(temp), "%s: %d\n", words[i], count[i]);
                strncpy(ptr[j], temp, strlen(temp));
                ptr[j] += strlen(temp);
                break;
            }
        }
        
        if (!found) {
            slot_name[slot_index] = strdup(slot);
            // buffer[slot_index] = (char *)malloc(bufferSize * sizeof(char));
            // if (buffer[slot_index] == NULL) {
            //     printf("alloc mem failed\n");
            //     return 1;
            // }
            memset(buffer[slot_index], 0, bufferSize * sizeof(char));
            char temp[100];
            snprintf(temp, sizeof(temp), "%s: %d\n", words[i], count[i]);
            strncpy(ptr[slot_index], temp, strlen(temp));
            ptr[slot_index] += strlen(temp);
            // sprintf(buffer[slot_index], "%s: %d\n", words[i], count[i]);
            // printf("slot: %s; buffer: %s\n", slot_name[slot_index], buffer[slot_index]);
            slot_index++;
        }
        // printf("found: %d; word: %s; count: %d; slot_name: %s; buffer: %s\n", found, words[i], count[i], slot, buffer[slot_index]);
        free(words[i]);
    }

    for (int i = 0; i < reducer_num; i++) {
        *ptr[i] = '\0';
    }

    // printf("buffer: %s\n", buffer[0]);

    for (int i = 0; i < slot_index; i++) {
        buffer_register(slot_name[i], strlen(slot_name[i]), buffer[i], bufferSize);
    }
    
    for (int i = 0; i < slot_index; i++) {
        free(slot_name[i]); // 释放 strdup 分配的内存
        free(buffer[i]);    // 释放 buffer
    }
    // write(1, "mapper end!\n", sizeof("mapper end!\n"));
    // printf("mapper_%d finished!\n", id);
    return 0;
}