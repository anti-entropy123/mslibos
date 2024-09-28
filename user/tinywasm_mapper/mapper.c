#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
#define MAX_WORD_LENGTH 100
#define MAX_WORDS 1000
void to_lowercase(char *str) {
    for (int i = 0; str[i]; i++) {
        str[i] = tolower(str[i]);
    }
}
int main() {
    int id = 1;
    int reducer_num = 3;
    char input_file[30];
    sprintf(input_file, "little_fake_data_%d.txt", id);
    FILE *file = fopen(input_file, "r");
    if (!file) {
        perror("Failed to open input file");
        exit(EXIT_FAILURE);
    }

    printf("mapper input file: %s\n", input_file);

    int count[MAX_WORDS] = {0};
    char *words[MAX_WORDS];
    char word[MAX_WORD_LENGTH];
    int word_index = 0;
    while (fscanf(file, "%s", word) != EOF) {
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
    fclose(file);

    printf("map read success!\n");

    char output_file[30];
    for (int i = 0; i < word_index; i++) {
        int partition_index = i % reducer_num;
        sprintf(output_file, "buffer_%d_%d.txt", partition_index, id);
        
        FILE *output = fopen(output_file, "a");
        fprintf(output, "%s %d\n", words[i], count[i]);
        fclose(output);
        
        free(words[i]);
    }

    printf("mapper finished!\n");
    return 0;
}