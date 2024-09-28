#include <stdio.h>
#include <stdlib.h>
#include <string.h>

__attribute__((import_module("env"), import_name("access_buffer"))) void access_buffer(void *slot_name, int name_size, void *buffer, int buffer_size);

int main()
{
    int bufferSize = 10;
    char *buffer;

    buffer = (char *)malloc(bufferSize * sizeof(char));

    if (buffer == NULL)
    {
        printf("alloc mem failed\n");
        return 1;
    }

    printf("alloc mem\n");
    char *slot_name = "slot_1";
    access_buffer(slot_name, strlen(slot_name), buffer, bufferSize);

    // for (int i = 0; i < bufferSize; i++)
    // {
    //     buffer[i] = 'A' + i;
    // }

    printf("c_app recv_str buffer: ");
    for (int i = 0; i < bufferSize; i++)
    {
        printf("%c ", buffer[i]);
    }
    printf("\n");

    // free(buffer);
}