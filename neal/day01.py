#!/usr/bin/env python3

from aoclib import read_input
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
    list_of_all_totals = map(total_calories, groups)
    # can not get map to turn into a list so....
    newlist = []
    for total in list_of_all_totals:
        newlist.append(total)
    #sort and cut and kill and eat
    return sum(sorted(newlist)[-3:])


if __name__ == "__main__":
    day_input = read_input(f"./input/{sys.argv[0][:-2]}txt")
    print(type(day_input))
    print(f"{sys.argv[0][:-3]}, part 1: {part1(day_input)}")
    print(f"{sys.argv[0][:-3]}, part 2: {part2(day_input)}")
