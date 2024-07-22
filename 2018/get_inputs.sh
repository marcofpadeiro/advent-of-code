#!/bin/bash
session=""

mkdir input

for i in {1..25}
do
    curl -H "Cookie:session=$session" https://adventofcode.com/2018/day/$i/input > "input/day$i.txt"
done
