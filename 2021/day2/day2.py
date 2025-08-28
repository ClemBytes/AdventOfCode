def read_input(f):
    exemple = ["forward 5", "down 5", "forward 8", "up 3", "down 8", "forward 2"]
    # return exemple
    return f.read().splitlines()


def puzzle1(cmds):
    h = 0
    d = 0
    for cmd in cmds:
        dir, nb = cmd.split(' ')
        nb = int(nb)
        if dir == "forward":
            h += nb
        elif dir == "down":
            d += nb
        elif dir == "up":
            d -= nb
        else:
            raise ValueError(f"Unkwnon direction: {dir}")
    print(d * h)


def puzzle2(cmds):
    h = 0
    d = 0
    aim = 0
    for cmd in cmds:
        dir, nb = cmd.split(' ')
        nb = int(nb)
        if dir == "forward":
            h += nb
            d += nb * aim
        elif dir == "down":
            aim += nb
        elif dir == "up":
            aim -= nb
        else:
            raise ValueError(f"Unkwnon direction: {dir}")
    print(d * h)


def main():
    # Read input
    f = open("/home/linaewen/PycharmProjects/advent2021/day2/input")
    cmds = read_input(f)
    # print(cmds)

    puzzle1(cmds)
    puzzle2(cmds)


main()
