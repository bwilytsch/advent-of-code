from pathlib import Path

file = Path('day_9_input.txt').read_text()

# Credit: https://www.youtube.com/watch?v=5_GstsPDI-Q
def part_two(input: str) -> int:
    files = {}
    blanks = []

    fid = 0
    pos = 0

    for i, char in enumerate(input):
        x = int(char)
        if i % 2 == 0:
            files[fid] = (pos, x)
            fid += 1
        else:
            if x != 0:
                blanks.append((pos, x))
        pos += x

    while fid > 0:
        fid -= 1
        pos, size = files[fid]
        for i, (start, length) in enumerate(blanks):
            if start >= pos:
                blanks = blanks[:i]
                break
            if size <= length:
                files[fid] = (start, size)
                if size == length:
                    blanks.pop(i)
                else:
                    blanks[i] = (start + size, length - size)
                break

    total = 0

    for fid, (pos, size) in files.items():
        for x in range(pos, pos + size):
            total += fid * x

    return total



print("Day 9 | Part 2", part_two(file.strip()))
