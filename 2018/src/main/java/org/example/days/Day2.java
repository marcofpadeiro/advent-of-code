package org.example.days;

import org.example.Day;

import java.util.HashMap;

public class Day2 extends Day<Integer, String> {
    public Day2() {
        super(2);
    }

    public Integer part1() {
        int twoCount = 0;
        int threeCount = 0;

        for (String line : this.input) {
            HashMap<Character, Integer> map = new HashMap<>();
            line.chars().forEach(c -> {
                map.put((char) c, map.getOrDefault((char) c, 0) + 1);
            });
            boolean hasTwo = false;
            boolean hasThree = false;

            for (int count : map.values()) {
                if (count == 2) {
                    hasTwo = true;
                }
                if (count == 3) {
                    hasThree = true;
                }
            }

            if (hasTwo) {
                twoCount++;
            }
            if (hasThree) {
                threeCount++;
            }
        }

        return twoCount * threeCount;
    }

    public String part2() {

        for (int i = 0; i < this.input.size(); i++) {
            for (int j = i + 1; j < this.input.size(); j++) {
                String current = this.input.get(i);
                String next = this.input.get(j);

                int diffCount = 0;
                int diffIndex = -1;

                for (int k = 0; k < current.length(); k++) {
                        if (current.charAt(k) != next.charAt(k)) {
                            diffCount++;
                            diffIndex = k;
                            if (diffCount > 1) break;
                        }
                }
                if (diffCount == 1) {
                    return current.substring(0, diffIndex) + next.substring(diffIndex + 1);
                }
            }
        }
        return null;
    }
}
