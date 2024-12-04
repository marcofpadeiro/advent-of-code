#include <limits.h>
#include <stdarg.h>
#include <stdio.h>

int part1();
int part2();
int smallest_side(int elements, ...);

int main(void) {
    printf("%d\n", part1());
    printf("%d\n", part2());
    return 0;
}

int part1() {
    FILE *fptr;
    fptr = fopen("../inputs/day2.txt", "r");

    char line[20];
    int total = 0;

    while (fgets(line, 20, fptr) != NULL) {
        int length = 0, width = 0, height = 0;
        sscanf(line, "%dx%dx%d", &length, &width, &height);

        int x = length * width;
        int y = width * height;
        int z = height * length;

        total += smallest_side(3, x, y, z) + x * 2 + y * 2 + z * 2;
    }
    printf("%d", total);
    fclose(fptr);
    return total;
}

int part2() {
    FILE *fptr;
    fptr = fopen("../inputs/day2.txt", "r");

    char line[20];
    int total = 0;

    while (fgets(line, 20, fptr) != NULL) {
        int elements[3] = {0, 0, 0};
        sscanf(line, "%dx%dx%d", &elements[0], &elements[1], &elements[2]);

        int area[3] = {elements[0] * elements[1], elements[1] * elements[2],
                       elements[2] * elements[1]};

        total += smallest_side(3, x, y, z) + x * 2 + y * 2 + z * 2;
    }
    printf("%d", total);
    fclose(fptr);
    return total;
}

int smallest_side(int count, ...) {
    va_list args;
    va_start(args, count);

    int min = INT_MAX;
    for (int i = 0; i < count; i++) {
        int num = va_arg(args, int);
        if (num < min) {
            min = num;
        }
    }

    va_end(args);
    return min;
}
