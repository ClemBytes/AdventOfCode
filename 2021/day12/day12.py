def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day12/{kind}") as f:
        entries = f.read().strip().split(' | ')


def puzzle1(entries):
    pass


def puzzle2(entries):
    pass


def main():
    # Read input
    entries = read_input("exemple")
    #print(entries)
    #print(puzzle1(entries))
    #print(puzzle2(entries))


main()
