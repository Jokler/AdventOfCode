#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

const char* numbers[] = {
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",

    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
};

const int number_lens[] = {
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,
    1,

    3,
    3,
    5,
    4,
    4,
    3,
    5,
    5,
    4
};

int starting_number(char* str) {
    for (int i = 0; i < 9; i++) {
        if (strncmp(numbers[i], str, number_lens[i]) == 0) {
            return i % 9 + 1;
        }
    }

    return -1;
}

int main(void)
{

    FILE* fp;
    char* line = NULL;
    size_t len = 0;
    ssize_t read;

    fp = fopen("input.txt", "r");
    if (fp == NULL)
        exit(EXIT_FAILURE);

    clock_t start = clock();
    int sum = 0;
    while ((read = getline(&line, &len, fp)) != -1) {
        int first = -1;
        int last = -1;
        for (int i = 0; line[i] != 0; i++) {
            int found = starting_number(&line[i]);
            if (found != -1) {
                if (first == -1) {
                    first = found;
                }
                last = found;
            }
        }
        if (first != -1) {
            sum += first * 10 + last;
        }
    }

    clock_t end = clock();
    float seconds = (float)(end - start) / CLOCKS_PER_SEC;

    printf("Micros: %f\n", seconds * 1000000);

    printf("%d\n", sum);

    fclose(fp);
    if (line)
        free(line);

    exit(EXIT_SUCCESS);
}
