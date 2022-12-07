from aoclib import read_input


def points(battle):
    # print(battle)
    # A Op_Rock B Op_Paper C Op_Scissors
    # X Me_Rock Y 1 Me_Paper Z 1 Me_Scissors 3
    # Loss 0, Draw 3, Win 6
    match battle:
        case "A X":
            result = "Op_Rock Me_Rock"
            # Move Points = 1
            # Draw Points = 3
            points = 4
            return points
        case "A Y":
            result = "Op_Rock Me_Paper"
            # Move Points = 2
            # Win Points = 6
            points = 8
            return points
        case "A Z":
            result = "Op_Rock Me_Scissors"
            # Move Points = 3
            # Loss Points = 0
            points = 3
            return points

        case "B X":
            result = "Op_Paper Me_Rock"
            # Move Points = 1
            # Loss Points = 0
            points = 1
            return points
        case "B Y":
            result = "Op_Paper Me_Paper"
            # Move Points = 1
            # Draw Points = 3
            points = 4
            return points
        case "B Z":
            result = "Op_Paper Me_Scissors"
            # Move Points = 3
            # Win Points = 6
            points = 9
            return points

        case "C X":
            result = "Op_Scissors Me_Rock"
            # Move Points = 1
            # Loss Points = 6
            points = 7
            return points
        case "C Y":
            result = "Op_Scissors Me_Paper"
            # Move Points = 2
            # Win Points = 0
            points = 2
            return points
        case "C Z":
            result = "Op_Scissors Me_Scissors"
            # Move Points = 3
            # Draw Points = 3
            points = 6
            return points


def part1(input):
    input = input.rstrip().split("\n")
    total = 0
    for battle in input:
        total = total + points(battle)
    return total


def part2(input):
    pass

if __name__ == "__main__":
    day_input = read_input(f"./input/{sys.argv[0][:-2]}txt")
    print(f"{sys.argv[0][:-3]}, part 1: {part1(day_input)}")
    print(f"{sys.argv[0][:-3]}, part 2: {part2(day_input)}")