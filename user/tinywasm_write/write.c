#include <unistd.h>

int main()
{
    write(1, "Hello World!\n", sizeof("Hello World!\n"));
    return 0;
}