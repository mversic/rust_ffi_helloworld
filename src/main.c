#include <stdio.h>  // For printf, fgets
#include <string.h> // For strcspn (to remove newline from fgets)
#include "hello.h"  // Include our new header file

int main()
{
    char nameBuffer[100];    // Buffer to store the input name
    char messageBuffer[200]; // Buffer for the output message from hello()

    printf("Enter a name: ");
    if (fgets(nameBuffer, sizeof(nameBuffer), stdin) != NULL)
    {
        // Remove trailing newline character, if present
        nameBuffer[strcspn(nameBuffer, "\n")] = 0;

        // Call the updated hello function, passing our output buffer.
        int result = hello(nameBuffer, messageBuffer, sizeof(messageBuffer));

        if (result >= 0)
        {
            // Success! Print the message from our buffer.
            printf("%s", messageBuffer);
        }
        else
        {
            // Handle the error case (e.g., buffer too small)
            fprintf(stderr, "Error: Could not generate greeting message.\n");
            return 1;
        }
    }
    else
    {
        // Handle fgets error or EOF
        fprintf(stderr, "Error reading input or end of file reached.\n");
        return 1; // Indicate an error
    }
    return 0;
}