#!/usr/bin/env python3

from aoclib import read_input

simon_input = "hej med dig"
test_input = read_input("../inputs/day01.txt")

# print(simon_input.split(" "))
# print("4000\n5000".split("\n"))

groups = test_input.split("\n\n")

def total_calories(group):
    # example: group == '1000\n2000\n3000'

    # 1. split string into list of items
    items = group.split("\n")

    # 2. convert list of strings into list of numbers
    items_for_real = map(int, items)

    # 3. summarise the items
    total = sum(items_for_real)

    return total

# total_calories(groups[0])
# total_calories(groups[1])

print("These are all the groups:")
for group in groups:
    result = total_calories(group)
    print(result)


#some_group = [1000,2000,3000]
#print(sum(some_group))

list_of_all_totals = map(total_calories, groups)

print("This is the elf who has double chin:):")
print(max(list_of_all_totals))