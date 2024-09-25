#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((import_module("env"), import_name("buffer_register"))) 
void buffer_register(void *slot_name, int name_size, void *buffer, int buffer_size);

int main()
{
    int bufferSize = 10; // 假设我们需要一个大小为10的缓冲区
    char *buffer;

    // 使用malloc分配内存
    buffer = (char *)malloc(bufferSize * sizeof(char));

    // 检查内存分配是否成功
    if (buffer == NULL)
    {
        printf("内存分配失败！\n");
        return 1;
    }
    char *slot_name = "slot_1";
    buffer_register(slot_name, strlen(slot_name), buffer, bufferSize);

    // // 对缓冲区进行一些操作，例如填充数据
    // for (int i = 0; i < bufferSize; i++)
    // {
    //     buffer[i] = 'A' + i;
    // }

    // // 打印缓冲区的内容
    // printf("缓冲区内容: ");
    // for (int i = 0; i < bufferSize; i++)
    // {
    //     printf("%c ", buffer[i]);
    // }
    // printf("\n");

    // // 使用free释放内存
    // free(buffer);

    return 0;
}