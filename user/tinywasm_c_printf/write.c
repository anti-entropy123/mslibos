#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <unistd.h>
int main()
{
    char input_file [20];
    sprintf(input_file, "fake_data_%d.txt", 1);
    printf("input file: %s\n", input_file);
    return 0;
}