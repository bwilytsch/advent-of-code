import os

def part_one(input: str) -> int:
    left, right = [], []

    original = [x for sublist in [[int(x) for x in line.split()] for line in input.strip().splitlines()] for x in sublist]
    left, right = original[::2], original[1::2]

    return sum(abs(x - y) for x,y in zip(sorted(left), sorted(right)))

def part_two(input: str) -> int:
    lines = input.strip().split('\n')

    lot = {}
    left = []

    for line in lines:
        (a,b) = list(filter(len, line.strip().split(" ")))

        left.append(int(a))

        if b in lot:
            lot[b] += 1
        else:
            lot[b] = 1

    sum = 0

    for value in left:
        if str(value) in lot:
            sum += value * lot[str(value)]

    return sum

dir = os.path.dirname(__file__);
input = open(os.path.join(dir, "day_1_input.txt")).read()

print("Day 1 | Part 1: ", part_one(input))
print("Day 1 | Part 2", part_two(input))
