def read_input(kind):
    # "kind" must be "input" or "exemple"
    # 1) Get draw :
    f_draw = open(f"/home/linaewen/PycharmProjects/advent2021/day4/{kind}_draw")
    draw = list(map(int, f_draw.read().split(',')))

    # 2) Get grids :
    f = open(f"/home/linaewen/PycharmProjects/advent2021/day4/{kind}")
    grids = [[]]
    marked = []
    count = 0
    for line in f.read().splitlines():
        if line == '':
            grids.append([])
            count += 1
            marked.append([[0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0],
                           [0, 0, 0, 0, 0]])
        else:
            #print(line.split())
            grids[count].append(list(map(int, line.split())))
    marked.append([[0, 0, 0, 0, 0],
                   [0, 0, 0, 0, 0],
                   [0, 0, 0, 0, 0],
                   [0, 0, 0, 0, 0],
                   [0, 0, 0, 0, 0]])

    return draw, grids, marked


def print_grid(nb, grids):
    print(f'\n Grid n°{nb} :')
    for line in grids[nb]:
        print(line)
    print(grids[nb])
    print('\n')


def check_win(nb, marked):
    for line in marked[nb]:
        if sum(line) == 5:
            return True

    for col in range(5):
        nb_marked_col = 0
        for l in range(5):
            if marked[nb][l][col] == 1:
                nb_marked_col += 1
        if nb_marked_col == 5:
            return True

    return False


def puzzle1(draw, grids, marked):
    nb_grids = len(grids)
    for number in draw:

        # Mark grids:
        for grid_nb in range(len(grids)):
            for line in range(5):
                for col in range(5):
                    if grids[grid_nb][line][col] == number:
                        marked[grid_nb][line][col] = 1

        # Check grids
        for grid_nb in range(nb_grids):
            if check_win(grid_nb, marked):
                #print_grid(grid_nb, grids)
                #print_grid(grid_nb, marked)
                sum_unmarked = 0
                for line in range(5):
                    for col in range(5):
                        if marked[grid_nb][line][col] == 0:
                            sum_unmarked += grids[grid_nb][line][col]
                return number * sum_unmarked


def puzzle2(draw, grids, marked):
    nb_grids = len(grids)
    no_win_list = list(range(nb_grids))
    number_place = 0
    while len(no_win_list) > 1:

        # Mark grids:
        for grid_nb in no_win_list:
            for line in range(5):
                for col in range(5):
                    if grids[grid_nb][line][col] == draw[number_place]:
                        marked[grid_nb][line][col] = 1

        # Check grids
        no_win_list = [
            grid_nb
            for grid_nb in no_win_list
            if not check_win(grid_nb, marked)
        ]

        number_place += 1

    # Compute score
    sum_unmarked = 0
    for line in range(5):
        for col in range(5):
            if marked[no_win_list[0]][line][col] == 0:
                sum_unmarked += grids[no_win_list[0]][line][col]
    sum_unmarked -= draw[number_place]
    print(grids[no_win_list[0]], marked[no_win_list[0]])
    print(f'{draw[number_place]=} {no_win_list=} {sum_unmarked=}')
    return draw[number_place] * sum_unmarked


def main():
    # Read input
    draw, grids, marked = read_input("input")
    """
    print(draw)
    print(len(grids))
    print(len(marked))
    print('\n')
    print_grid(grids[0])
    print('\n')
    print_grid(marked[0])
    print('\n')
    print(check_win(marked[0]))
    print(check_win([[0, 0, 0, 0, 0], [0, 1, 0, 0, 0], [0, 1, 0, 0, 0], [0, 1, 0, 0, 0], [0, 1, 0, 0, 0]]))
    """
    #print(puzzle1(draw, grids, marked))
    print(puzzle2(draw, grids, marked))


main()
