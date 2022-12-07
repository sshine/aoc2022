
def read_input(input_path):
    with open(input_path) as file:
        return file.read()


def print_solutions(day, part1, part2):
    input = read_input(f"./input/day{day}.txt")
    print("Day {}, part 1: {}".format(day, part1(input)))
    print("Day {}, part 2: {}".format(day, part2(input)))
