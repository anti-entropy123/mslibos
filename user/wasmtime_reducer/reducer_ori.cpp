#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

__attribute__((import_module("env"), import_name("access_buffer"))) void access_buffer(void *slot_name, int name_size, void *buffer, int buffer_size);

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
int main(int argc, char* argv[]) {
    int id = atoi(argv[1]);
    int mapper_num = atoi(argv[2]);
    printf("reducer.c recieve: id: %d, mapper_num: %d\n", id, mapper_num);

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
    int slot_num = mapper_num;

    printf("access start!\n");

    for (int i = 0; i < slot_num; ++i) {
        sprintf(slot, "buffer_%d_%d", id, i);
        slot_name[i] = strdup(slot);
        buffer[i] = (char *)malloc(bufferSize * sizeof(char));
        if (buffer[i] == NULL) {
            printf("alloc mem failed\n");
            return 1;
        }
        access_buffer(slot_name[i], strlen(slot_name[i]), buffer[i], bufferSize);
    }
    printf("access success!\n");

    char word[MAX_WORD_LENGTH];
    int word_index = 0;

    char *ptr[MAX_SLOT_NUM];
    for (int i = 0; i < slot_num; i++) {
        ptr[i] = buffer[i];
    }
    int num;
    for (int i = 0; i < slot_num; i++) {
        while (sscanf(ptr[i], "%s: %d\n", word, &num) == 1) {
            to_lowercase(word);
            int found = 0;
            for (int j = 0; j < word_index; j++) {
                if (strcmp(words[j], word) == 0) {
                    found = 1;
                    count[j] += num;
                    break;
                }
            }
            if (!found) {
                words[word_index] = strdup(word);
                count[word_index] += num;
                word_index++;
            }
            
            // 移动指针到下一个char
            while (*ptr[i] && *ptr[i] != ' ') {
                ptr[i]++;
            }
            if (*ptr[i] == ' ') {
                ptr[i]++;
            }
        }
    }

    printf("reducer_%d_index: %d\n", id, word_index);
    printf("reducer_%d read success!\n", id);

    char output_file[30];
    sprintf(output_file, "reducer_%d.txt", id);
    FILE *output = fopen(output_file, "w");
    for (int i = 0; i < word_index; i++) {
        fprintf(output, "%s %d\n", words[i], count[i]);
        free(words[i]);
    }
    
    fclose(output);
    
    printf("reducer_%d finished!\n", id);
    return 0;
}