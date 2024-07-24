package org.example.days;

import org.example.Day;

import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day4 extends Day<Integer, Integer> {
    private static final Pattern date = Pattern.compile("\\[(\\d+)-(\\d{2})-(\\d{2}) (\\d{2}):(\\d{2})\\]");
    private static final Pattern shift = Pattern.compile("\\[(\\d+)-(\\d{2})-(\\d{2}) (\\d{2}):(\\d{2})\\] Guard #(\\d+) begins shift");

    public Day4() {
        super(4);
        Collections.sort(input);
    }

    public Integer part1() {
        HashMap<Integer, int[]> tracker = processTracker();

        int sleepiestGuard = findSleepiestGuard(tracker);
        int[] sleepiestHour = findSleepiestHour(tracker.get(sleepiestGuard));


        return sleepiestGuard * sleepiestHour[0];

    }

    public Integer part2() {
        HashMap<Integer, int[]> tracker = processTracker();

        int mostSleptHourCounter = 0;
        int mostSleptHour = 0;
        int guardID = 0;

        for (Map.Entry<Integer, int[]> entry : tracker.entrySet()) {
            var temp = findSleepiestHour(entry.getValue());

            if (temp[1] > mostSleptHourCounter) {
                mostSleptHourCounter = temp[1];
                mostSleptHour = temp[0];
                guardID = entry.getKey();
            }
        }
        return mostSleptHour * guardID;
    }

    private HashMap<Integer, int[]> processTracker() {
        HashMap<Integer, int[]> tracker = new HashMap<>();

        Integer sleepStart = 0;
        Integer currentGuard = 0;


        for (String l : input) {
            Line line = new Line(l);

            switch (line.action) {
                case Action.SHIFT:
                    currentGuard = line.guardID;
                    tracker.putIfAbsent(currentGuard, new int[60]);
                    break;
                case Action.SLEEP:
                    sleepStart = line.minute;
                    break;
                case Action.WAKEUP:
                    for (int i = sleepStart; i < line.minute; i++) {
                        tracker.get(currentGuard)[i]++;
                    }
                    break;
            }

        }

        return tracker;
    }

    private int findSleepiestGuard(HashMap<Integer, int[]> tracker) {
        int maxGuard = 0;
        int maxMinutesAsleep = 0;
        for (Map.Entry<Integer, int[]> entry : tracker.entrySet()) {
            int totalMinutesAsleep = Arrays.stream(entry.getValue()).sum();
            if (totalMinutesAsleep > maxMinutesAsleep) {
                maxGuard = entry.getKey();
                maxMinutesAsleep = totalMinutesAsleep;
            }
        }
        return maxGuard;

    }

    private int[] findSleepiestHour(int[] sleepMinutes) {
        int maxMinute[] = {0, 0};
        for (int i = 0; i < sleepMinutes.length; i++) {
            if (sleepMinutes[i] > maxMinute[1]) {
                maxMinute[0] = i;
                maxMinute[1] = sleepMinutes[i];
            }
        }

        return maxMinute;
    }

    private enum Action {
        SHIFT,
        SLEEP,
        WAKEUP
    }

    private class Line {
        int month;
        int day;
        int hour;
        int minute;
        Action action;
        int guardID;

        public Line(String line) {
            Matcher matcher;
            if (line.contains("wake")) {
                action = Action.WAKEUP;
                matcher = date.matcher(line);

            } else if (line.contains("asleep")) {
                action = Action.SLEEP;
                matcher = date.matcher(line);
            } else {
                action = Action.SHIFT;
                matcher = shift.matcher(line);
            }
            if (matcher.find()) {
                month = Integer.parseInt(matcher.group(2));
                day = Integer.parseInt(matcher.group(3));
                hour = Integer.parseInt(matcher.group(4));
                minute = Integer.parseInt(matcher.group(5));

                guardID = this.action == Action.SHIFT ? Integer.parseInt(matcher.group(6)) : 0;
            }
        }
    }
}
