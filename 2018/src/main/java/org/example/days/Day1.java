package org.example.days;

import org.example.Day;

import java.util.HashSet;

public class Day1 extends Day<Integer, Integer> {
    public Day1() {
        super(1);
    }

    public Integer part1() {
        return this.input.stream()
                .map(Integer::valueOf)
                .mapToInt(Integer::intValue)
                .sum();
    }

    public Integer part2() {
        HashSet<Integer> res= new HashSet<>();
        int i = 0;
        res.add(i);

        while (true) {
            for (String line : this.input) {
                i += Integer.parseInt(line);

                if (!res.add(i)) {
                    return i;
                }
            }
        }
    }
}
