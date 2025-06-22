#include "hello.h" // Include our header file
#include <stdio.h> // For snprintf

const char *hello(const char *name)
{
    static char messageBuffer[300]; // Buffer to store the message

    // Use "World" as a default if the provided name is NULL or empty.
    const char *effective_name = (name != NULL && name[0] != '\0') ? name : "World";
    snprintf(messageBuffer, sizeof(messageBuffer), "Hello, %s\n", effective_name);

    return messageBuffer;
}