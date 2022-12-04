#!/usr/bin/env python3

from aoclib import read_input

def part1(input):
    groups = input.split("\n\n")
    list_of_all_totals = map(total_calories, groups)

    return max(list_of_all_totals)

def total_calories(group):
    # 1. split string into list of items
    items = group.split("\n")

    # 2. convert list of strings into list of numbers
    calories = map(int, items)

    # 3. summarise the calories
    total = sum(calories)

    return total

def part2(input):
    pass

if __name__ == "__main__":
    day1_input = read_input("../inputs/day01.txt")
    print(f"Day 1, part 1: This is the elf who has double chin:)): {part1(day1_input)}")
    print(f"Day 1, part 2: {part2(day1_input)}")