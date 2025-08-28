def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day6/{kind}") as f:
        return [int(x) for x in f.read().strip().split(',')]


def next_state(fishes):
    next_fishes = []
    for fish in fishes:
        if fish == 0:
            next_fishes.append(6)
            next_fishes.append(8)
        else:
            next_fishes.append(fish - 1)
    return next_fishes


def puzzle1(fishes):
    next_fishes = fishes
    for _ in range(80):
        next_fishes = next_state(next_fishes)
    return len(next_fishes)


def puzzle2(fishes):
    d = {
        0: 0,
        1: 0,
        2: 0,
        3: 0,
        4: 0,
        5: 0,
        6: 0,
        7: 0,
        8: 0,
    }
    for fish in fishes:
        d[fish] += 1

    for _ in range(256):
        reset = d[0]
        d[0] = d[1]
        d[1] = d[2]
        d[2] = d[3]
        d[3] = d[4]
        d[4] = d[5]
        d[5] = d[6]
        d[6] = d[7] + reset
        d[7] = d[8]
        d[8] = reset

    return(sum(d.values()))


def main():
    # Read input
    fishes = read_input("input")
    #print(fishes)
    print(puzzle1(fishes))
    print(puzzle2(fishes))


main()
