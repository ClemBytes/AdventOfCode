from itertools import permutations as perm

def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day8/{kind}") as f:
        entries = []
        for line in f:
            p, o = line.strip().split(' | ')
            patterns = p.split()
            output = o.split()
            entries.append([patterns, output])
        return entries


def puzzle1(entries):
    count = 0
    for entry in entries:
        _, output = entry
        for o in output:
            if len(o) == 2 or len(o) == 3 or len(o) == 4 or len(o) == 7:
                count += 1
    return count


def segments_to_digit(segments):
    digits = {
        0: "abcefg",
        1: "cf",
        2: "acdeg",
        3: "acdfg",
        4: "bcdf",
        5: "abdfg",
        6: "abdefg",
        7: "acf",
        8: "abcdefg",
        9: "abcdfg",
    }
    for d in digits:
        if set(segments) == set(digits[d]):
            return d
    #print(f"Unkwon digit: {segments}")
    return None


def apply_permutation(segment, permutation):
    d = {
        'a': permutation[0],
        'b': permutation[1],
        'c': permutation[2],
        'd': permutation[3],
        'e': permutation[4],
        'f': permutation[5],
        'g': permutation[6],
    }
    permuted = ''
    for letter in segment:
        permuted += d[letter]
    return permuted


def find_permutation(patterns):
    possible_permutations = list(perm("abcdefg"))
    for permutation in possible_permutations:
        count = 0
        for p in patterns:
            if segments_to_digit(apply_permutation(p, permutation)) is not None:
                count += 1
        if count == 10:
            return permutation


def puzzle2(entries):
    s = 0
    for entry in entries:
        patterns, output = entry
        permutation = find_permutation(patterns)
        output_value = ''
        for o in output:
            output_value += str(segments_to_digit(apply_permutation(o, permutation)))
        s += int(output_value)
    return s


def main():
    # Read input
    entries = read_input("input")
    #print(entries)
    print(puzzle1(entries))
    print(puzzle2(entries))


main()
