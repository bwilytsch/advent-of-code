import os

def part_one(input: str) -> int:
    sum = 0
    lines = input.strip().split('\n')

    left = []
    right = []

    for line in lines:
        (a,b) = list(filter(len, line.strip().split(" ")))

        left.append(int(a))
        right.append(int(b))

    left.sort()
    right.sort()

    for i, value in enumerate(left):
       sum += abs(value - right[i])

    return sum

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

print(part_one(input))
print(part_two(input))
