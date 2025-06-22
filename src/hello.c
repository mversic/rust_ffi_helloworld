#include "hello.h" // Include our header file
#include <stdio.h> // For snprintf

int hello(const char *name, char *out_buffer, size_t buffer_len)
{
    // Use "World" as a default if the provided name is NULL or empty.
    const char *effective_name = (name != NULL && name[0] != '\0') ? name : "World";

    int written = snprintf(out_buffer, buffer_len, "Hello, %s\n", effective_name);

    if (written < 0 || (size_t)written >= buffer_len)
    {
        // An error occurred or the buffer was too small.
        return -1;
    }

    return written;
}