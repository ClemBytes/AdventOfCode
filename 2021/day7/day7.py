def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day7/{kind}") as f:
        return [int(x) for x in f.read().strip().split(',')]


def cost(align, positions):
    return sum(abs(pos - align) for pos in positions)


def cost_bis(align, positions):
    s = 0
    for pos in positions:
        diff = abs(pos - align)
        s += (diff * (diff + 1))/2
    return s


def puzzle1(positions):
    max_position = max(positions)
    costs = []
    for align in range(max_position + 1):
        costs.append(cost(align, positions))
    best_alignment = min(range(len(costs)), key=costs.__getitem__)
    return cost(best_alignment, positions)


def puzzle2(positions):
    max_position = max(positions)
    costs = []
    for align in range(max_position + 1):
        costs.append(cost_bis(align, positions))
    best_alignment = min(range(len(costs)), key=costs.__getitem__)
    return cost_bis(best_alignment, positions)


def main():
    # Read input
    positions = read_input("input")
    """
    print(positions)
    for align in [2, 1, 3, 10]:
        print(f'{align=} cost={cost(align, positions)}')
    """
    for align in [5, 2]:
        print(f'{align=} cost={cost_bis(align, positions)}')

    print(puzzle1(positions))
    print(puzzle2(positions))


main()
