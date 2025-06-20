// We no longer need stdio.h here directly, as printf is in hello.c
#include "hello.h" // Include our new header file

int main()
{
    hello(); // Call the function from hello.c
    return 0;
}