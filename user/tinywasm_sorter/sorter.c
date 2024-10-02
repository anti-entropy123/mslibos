#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((import_module("env"), import_name("buffer_register"))) void buffer_register(void *slot_name, int name_size, void *buffer, int buffer_size);

#define MAX_ARRAY_LENGTH 100

// 比较函数，用于 qsort
int compare(const void *a, const void *b) {
    return (*(int *)a - *(int *)b); // 升序排序
}

int main(int argc, char* argv[]) {
    int id = atoi(argv[1]);
    int sorter_num = atoi(argv[2]);
    int merger_num = atoi(argv[3]);
    printf("sorter_%d start!\n", id);
    
    char input_file[30];
    printf("sorter_%d file define!\n", id);
    sprintf(input_file, "sort_data_%d.txt", id);
    printf("sorter_%d fopen start!\n", id);
    FILE *file = fopen(input_file, "r");
    if (!file) {
        perror("Failed to open input file\n");
        exit(EXIT_FAILURE);
    }
    printf("sorter_%d fopen finished!\n", id);
    int array[MAX_ARRAY_LENGTH];
    // char number[20];
    int index = 0;
    // while (fscanf(file, "%s", number) != EOF) {
    //     array[index] = atoi(number);
    //     // printf("array[%d]: %d\n", index, array[index]);
    //     index++;
    // }
    int ch = fgetc(file), x = 0;
    for (; ch != '\n' && ch != EOF; ch = fgetc(file)) {
        if (ch == ',') array[index++] = x, x = 0; 
        else x = x * 10 + ch - '0';
    }
    array[index++] = x;
    
    
    // while (~fscanf(file, "%d,", &array[index])) {
    //     index++;
    //     if (index >= MAX_ARRAY_LENGTH) {
    //         break;
    //     }
    //     int ch = fgetc(file);
    //     if (ch != ',') {
    //         ungetc(ch, file); // 如果不是逗号，放回字符流，跳出循环
    //         break;
    //     }
    // }
    printf("sorter_%d read finished!\n", id);
    // printf("index: %d\n", index);
    fclose(file);
    qsort(array, index, sizeof(int), compare);

    printf("sorter_%d sort finished!\n", id);

    if (id == 0) {
        int pivot[merger_num-1];
        for (int i = 0; i < merger_num-1; i++) {
            int idx = (i+1) * index / merger_num;
            pivot[i] = array[idx];
        }
        for (int k = 0; k < sorter_num; k++) {
            char slot_name[20];
            sprintf(slot_name, "pivot_%d", k);
            // printf("pivotname: %s\n", slot_name);
            int bufferSize = 1000;
            char *buffer;
            buffer = (char *)malloc(bufferSize * sizeof(char));
            if (buffer == NULL) {
                perror("malloc error");
                return 1;
            }
            memset(buffer, 0, bufferSize * sizeof(char));
            buffer[0] = '\0'; // 初始化为空字符串
            for (int i = 0; i < merger_num-1; i++) {
                char temp[12]; // 临时缓冲区，注意要足够大以容纳最大整数和一个空格
                snprintf(temp, sizeof(temp), "%d ", pivot[i]); // 将整数转换为字符串，并加上空格
                strcat(buffer, temp); // 追加到 buffer
            }
            // 去掉最后一个多余的空格
            buffer[strlen(buffer) - 1] = '\0';
            buffer_register(slot_name, strlen(slot_name), buffer, bufferSize);
            free(buffer);
        }
    }

    // printf("sorter_%d pivot finished!\n", id);

    char slot_name[20];
    sprintf(slot_name, "sorter_%d", id);
    char *buffer;
    int bufferSize = 1000;
    // printf("buffersize: %d\n", bufferSize);
    buffer = (char *)malloc(bufferSize * sizeof(char));
    if (buffer == NULL) {
        perror("malloc error");
        return 1;
    }
    memset(buffer, 0, bufferSize * sizeof(char));
    buffer[0] = '\0'; // 初始化为空字符串
    for (int i = 0; i < index; i++) {
        char temp[12]; // 临时缓冲区，注意要足够大以容纳最大整数和一个空格
        snprintf(temp, sizeof(temp), "%d ", array[i]); // 将整数转换为字符串，并加上空格
        strcat(buffer, temp); // 追加到 buffer
    }
    // 去掉最后一个多余的空格
    buffer[strlen(buffer) - 1] = '\0';

    buffer_register(slot_name, strlen(slot_name), buffer, bufferSize);
    free(buffer);
    printf("sorter_%d all finished!\n", id);
    return 0;
}