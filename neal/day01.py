#!/usr/bin/env python3

from aoclib import read_input, print_solutions
import sys


def part1(input):
    groups = input.split("\n\n")
    list_of_all_totals = map(total_calories, groups)
    return max(list_of_all_totals)


def total_calories(group):
    # 1. split string into list of items
    items = group.rstrip().split("\n")
    # 2. convert list of strings into list of numbers
    calories = map(int, items)
    # 3. summarise the calories
    total = sum(calories)
    return total


def part2(input):
    groups = input.split("\n\n")
    return sum(sorted(map(total_calories, groups))[-3:])


if __name__ == "__main__":
    print_solutions("01", part1, part2)
