#include "hello.h" // Include our header file
#include <stdio.h> // For sprintf

const char *hello(const char *name)
{
    static char messageBuffer[300]; // Buffer to store the message
    sprintf(messageBuffer, "hello %s\n", name);

    return messageBuffer;
}