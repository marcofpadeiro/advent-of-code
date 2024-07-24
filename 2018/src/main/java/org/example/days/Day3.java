package org.example.days;

import org.example.Day;

import java.util.ArrayList;
import java.util.regex.Matcher;
import java.util.regex.Pattern;


public class Day3 extends Day<Integer, Integer> {
    private static final int MAX_SQUARE = 1000;
    private static final Pattern p = Pattern.compile("#(\\d+) @ (\\d+),(\\d+): (\\d+)x(\\d+)");
    public Day3() {
        super(3);
    }

    public Integer part1() {
        int[][] rectangle = new int[MAX_SQUARE][MAX_SQUARE];

        for (String line : input) {
            Line l = new Line(line);

            for (int i = 0; i < l.size_width; i++) {
                for (int j = 0; j < l.size_height; j++) {
                    rectangle[i + l.distance_width][j + l.distance_height]++;
                }
            }

        }

        int count = 0;
        for (int i = 0; i < MAX_SQUARE; i++) {
            for (int j = 0; j < MAX_SQUARE; j++) {
                if (rectangle[i][j] >= 2) {
                   count++;
                }
            }
        }

        return count;
    }

    public Integer part2() {
        int[][] rectangle = new int[MAX_SQUARE][MAX_SQUARE];
        ArrayList<Integer> list = new ArrayList<>();

        for (String l: input) {
            Line line = new Line(l);

            list.add(line.id);

            for (int i = 0; i < line.size_width; i++) {
                for (int j = 0; j < line.size_height; j++) {
                    if (rectangle[i + line.distance_width][j + line.distance_height] == 0) {
                        rectangle[i + line.distance_width][j + line.distance_height] = line.id;
                        continue;
                    }

                    list.remove(Integer.valueOf(line.id));
                    list.remove(Integer.valueOf(rectangle[i + line.distance_width][j + line.distance_height]));
                    rectangle[i + line.distance_width][j + line.distance_height] = -1;
                }
            }

        }

        return list.getFirst();
    }
    private class Line {
        int id;
        int distance_width;
        int distance_height;
        int size_width;
        int size_height;

        public Line(String line) {
            Matcher m = p.matcher(line);
            if (m.find()) {
                this.id = Integer.parseInt(m.group(1));
                this.distance_width = Integer.parseInt(m.group(2));
                this.distance_height = Integer.parseInt(m.group(3));
                this.size_width = Integer.parseInt(m.group(4));
                this.size_height = Integer.parseInt(m.group(5));
            }
        }
    }
}
