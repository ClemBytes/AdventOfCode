from copy import deepcopy

def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day11/{kind}") as f:
        energy = [list(map(int, line.strip())) for line in f]
        return energy


def print_grid(grid):
    for line in grid:
        print(*line)


def one_step(energy, NBL, NBC):
    flashes = 0
    for l in range(NBL):
        for c in range(NBC):
            energy[l][c] += 1
    for l in range(NBL):
        for c in range(NBC):
            if energy[l][c] > 9:
                flashes += 1
                # Séparer la détection des flash de leur application, en gardant une liste de ceux qui flashent
                for dl, dc in [(l - 1, c - 1),
                               (l, c - 1),
                               (l + 1, c - 1),
                               (l + 1, c),
                               (l + 1, c + 1),
                               (l, c + 1),
                               (l - 1, c + 1),
                               (l - 1, c)]:
                    if 0 <= dl < NBL and 0 <= dc < NBC:
                        energy[dl][dc] += 1



def puzzle1(energy, NBL, NBC):
    energy = deepcopy(energy)



def puzzle2(energy, NBL, NBC):
    pass


def main():
    # Read input
    energy = read_input("exemple_simple")
    print_grid(energy)
    #print(energy)
    NBL = len(energy)
    NBC = len(energy[0])
    #print(puzzle1(energy, NBL, NBC))
    #print(puzzle2(energy, NBL, NBC))


main()
