#include <stdio.h>  // For printf, fgets
#include <string.h> // For strcspn (to remove newline from fgets)
#include "hello.h"  // Include our new header file

int main()
{
    char nameBuffer[100]; // Buffer to store the input name

    printf("Enter a name: ");
    if (fgets(nameBuffer, sizeof(nameBuffer), stdin) != NULL)
    {
        // Remove trailing newline character, if present
        // strcspn returns the length of the initial segment of nameBuffer
        // which consists entirely of characters not in "\n"
        nameBuffer[strcspn(nameBuffer, "\n")] = 0;

        const char *helloMessage = hello(nameBuffer); // Call the function with the input name

        printf("%s", helloMessage); // Call the function with the input name
    }
    else
    {
        // Handle fgets error or EOF
        fprintf(stderr, "Error reading input or end of file reached.\n");
        return 1; // Indicate an error
    }
    return 0;
}