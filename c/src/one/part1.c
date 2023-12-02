#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>

int main(void)
{
    FILE* fp;
    char* line = NULL;
    size_t len = 0;
    ssize_t read;

    fp = fopen("input.txt", "r");
    if (fp == NULL)
        exit(EXIT_FAILURE);

    int sum = 0;
    while ((read = getline(&line, &len, fp)) != -1) {
        int first = -1;
        int last = -1;
        for (int i = 0; line[i] != 0; i++) {
            if (line[i] >= '0' && line[i] <= '9') {
                if (first == -1) {
                    first = line[i] - '0';
                }
                last = line[i] - '0';
            }
        }
        sum += first * 10 + last;
    }

    printf("%d\n", sum);

    fclose(fp);
    if (line)
        free(line);

    exit(EXIT_SUCCESS);
}
