// We no longer need stdio.h here directly, as printf is in hello.c
#include "hello.h" // Include our new header file

int main()
{
    const char *myName = "Alice";
    hello(myName); // Call the function from hello.c with a name
    return 0;
}