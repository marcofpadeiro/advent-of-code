#include <stdio.h>

int part1();
int part2();

int main(void) {
    printf("%d\n", part1());
    printf("%d\n", part2());
    return 0;
}

int part1() {
    FILE *fptr;
    fptr = fopen("../inputs/day1.txt", "r");

    int total = 0;
    char c;

    while ((c = fgetc(fptr)) != EOF)
        total += c == '(' ? 1 : -1;

    fclose(fptr);
    return total;
}

int part2() {
    FILE *fptr;
    fptr = fopen("../inputs/day1.txt", "r");

    int total = 0;
    int c = 0;
    int i = 0;

    while ((c = fgetc(fptr)) != EOF && total != -1) {
        total += c == '(' ? 1 : -1;
        i++;
    }

    fclose(fptr);

    return i;
}
