package org.example;

import java.lang.reflect.Constructor;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in)
                ;
        System.out.println("Enter day (1..25) [default: most recent]: ");

        String input = scanner.nextLine();

        int i;
        if (input.trim().isEmpty()) {
            i = findLatestDay();
        } else {
            try {
                i = Integer.parseInt(input.trim());
            } catch (NumberFormatException e) {
                System.out.println("Invalid input. Please enter a number between 1 and 25.");
                scanner.close();
                return;
            }
        }

        Day day = getDay(i);
        System.out.println(day);

        scanner.close();
    }

    private static Day getDay(int day) {
        Day dayInstance = null;
        try {
            String className = "org.example.days.Day" + day;
            Class<?> clazz = Class.forName(className);

            Constructor<?> constructor = clazz.getConstructor();
            dayInstance = (Day) constructor.newInstance();
        } catch (Exception e) {
            e.printStackTrace();
        }

        return dayInstance;
    }

    private static int findLatestDay() {
        int latestDay = 1;
        for (int i = 1; i <= 25; i++) {
            try {
                Class.forName("org.example.days.Day" + i);
                latestDay = i;
            } catch (ClassNotFoundException e) {
                break;
            }
        }
        return latestDay;
    }
}
