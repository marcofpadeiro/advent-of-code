package org.example;

import java.awt.print.Printable;
import java.io.File;
import java.io.FileNotFoundException;
import java.util.LinkedList;
import java.util.List;
import java.util.Scanner;

public abstract class Day<T, K> {
    public List<String> input;
    private int day;

    public Day(int day) {
        this.input = new LinkedList<String>();
        this.day = day;
        String filename = "input/day" + day + ".txt";

        try {
            File file = new File(filename);
            Scanner reader = new Scanner(file);
            while (reader.hasNextLine()) {
                input.add(reader.nextLine());
            }
            reader.close();
        } catch (FileNotFoundException e) {
            System.out.println("An error occurred.");
            e.printStackTrace();
        }
    }

    public abstract T part1();
    public abstract K part2();

    @Override
    public String toString() {
        long startTime = System.nanoTime();

        T output_1 = part1();

        long endTime = System.nanoTime();
        double part1 = (endTime - startTime) / 1_000_000.0;

        startTime = System.nanoTime();
        K output_2 = part2();
        endTime = System.nanoTime();
        double part2 = (endTime - startTime) / 1_000_000.0;

        return "Day " + day + "\n\tPart 1: " + output_1 + "(" + part1 + "ms)" + "\n\tPart 2: " + output_2 + "(" + part2 + "ms)";
    }
}
