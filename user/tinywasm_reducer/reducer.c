#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
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
    int count[MAX_WORDS] = {0};
    char *words[MAX_WORDS];
    char word[MAX_WORD_LENGTH];
    int word_index = 0;
    for (int i = 0; i < reducer_num; ++i) {
        char input_file[30];
        sprintf(input_file, "buffer_%d_%d.txt", i, id);
        FILE *file = fopen(input_file, "r");
        if (!file) {
            perror("Failed to open input file");
            exit(EXIT_FAILURE);
        }
        int num;
        while (fscanf(file, "%s %d", word, &num) != EOF) {
            to_lowercase(word);
            
            int found = 0;
            for (int i = 0; i < word_index; i++) {
                if (strcmp(words[i], word) == 0) {
                    count[i] += num;
                    found = 1;
                    break;
                }
            }
            
            if (!found) {
                words[word_index] = strdup(word);
                count[word_index] = num;
                word_index++;
            }
        }
    }
    char output_file[30];
    sprintf(output_file, "reducer_%d.txt", id);
    FILE *output = fopen(output_file, "w");
    for (int i = 0; i < word_index; i++) {
        fprintf(output, "%s %d\n", words[i], count[i]);
        free(words[i]);
    }
}