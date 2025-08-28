def print_grid(grid):
    for line in grid:
        print(''.join(line))


def new_layout1(layout):
    ret = []
    nb_lines = len(layout)
    nb_col = len(layout[0])
    for line in range(len(layout)):
        new_line = []
        for column in range(len(layout[line])):
            val = layout[line][column]
            if val == '.':
                new_line.append('.')
            else:
                occupied = 0
                for i in range(max(0, line - 1), min(nb_lines, line + 2)):
                    for j in range(max(0, column - 1), min(nb_col, column + 2)):
                        if layout[i][j] == '#' and not (i == line and j == column):
                            occupied += 1

                if val == 'L':
                    if occupied == 0:
                        new_line.append('#')
                    else:
                        new_line.append('L')

                elif val == '#':
                    if occupied >= 4:
                        new_line.append('L')
                    else:
                        new_line.append('#')

                else:
                    raise ValueError(f'Unknown position {val}')

        ret.append(new_line)

    return ret


def new_layout2(layout):
    ret = []
    nb_lines = len(layout)
    nb_col = len(layout[0])
    for line in range(len(layout)):
        new_line = []
        for column in range(len(layout[line])):
            val = layout[line][column]
            if val == '.':
                new_line.append('.')
            else:
                occupied = 0

                # 1 : ↑
                for i in reversed(range(line)):
                    if layout[i][column] == '#':
                        occupied += 1
                        break
                    elif layout[i][column] == 'L':
                        break

                # 2 : ↓
                for i in range(line + 1, nb_lines):
                    if layout[i][column] == '#':
                        occupied += 1
                        break
                    elif layout[i][column] == 'L':
                        break

                # 3 : ←
                for j in reversed(range(column)):
                    if layout[line][j] == '#':
                        occupied += 1
                        break
                    elif layout[line][j] == 'L':
                        break

                # 4 : →
                for j in range(column + 1, nb_col):
                    if layout[line][j] == '#':
                        occupied += 1
                        break
                    elif layout[line][j] == 'L':
                        break

                # 5 : ←↑
                i = line
                j = column
                while i > 0 < j:
                    i -= 1
                    j -= 1
                    if layout[i][j] == '#':
                        occupied += 1
                        break
                    elif layout[i][j] == 'L':
                        break

                # 6 : ↑→
                i = line
                j = column
                while i > 0 and j < nb_col - 1:
                    i -= 1
                    j += 1
                    if layout[i][j] == '#':
                        occupied += 1
                        break
                    elif layout[i][j] == 'L':
                        break

                # 7 : ←↓
                i = line
                j = column
                while i < nb_lines - 1 and j > 0:
                    i += 1
                    j -= 1
                    if layout[i][j] == '#':
                        occupied += 1
                        break
                    elif layout[i][j] == 'L':
                        break

                # 8 : ↓→
                i = line
                j = column
                while i < nb_lines - 1 and j < nb_col - 1:
                    i += 1
                    j += 1
                    if layout[i][j] == '#':
                        occupied += 1
                        break
                    elif layout[i][j] == 'L':
                        break

                if val == 'L':
                    if occupied == 0:
                        new_line.append('#')
                    else:
                        new_line.append('L')

                elif val == '#':
                    if occupied >= 5:
                        new_line.append('L')
                    else:
                        new_line.append('#')

                else:
                    raise ValueError(f'Unknown position {val}')

        ret.append(new_line)

    return ret


def loop(grid, n):
    old = grid
    while True:
        if n == 1:
            new = new_layout1(old)
        elif n == 2:
            new = new_layout2(old)
        else:
            raise ValueError(f'Choose method 1 or 2, not: {n}')

        if old == new:
            return new
        else:
            old = new


def count_occupied(grid):
    return sum(line.count('#') for line in grid)


def main():
    # Read input
    f = open("./input")
    grid = [list(line.strip()) for line in f]
    print(count_occupied(loop(grid, 2)))


main()
