#ifndef HELLO_H
#define HELLO_H

#include <stddef.h> // For size_t

// Writes a greeting into the provided buffer.
// Returns the number of bytes written (excluding the null terminator), or -1 on error.
int hello(const char *name, char *out_buffer, size_t buffer_len);

#endif // HELLO_H