def compute_id(row, col):
    return 8 * row + col


def find_row(bp):
    mi = 0
    ma = 127
    for l in bp[:6]:
        if l == 'F':
            ma = mi + (ma - mi) // 2
        else:  # l == 'B'
            mi = mi + (ma - mi) // 2 + 1
    if bp[6] == 'F':
        return mi
    else:  # bp[7] == 'B':
        return ma


def find_col(bp):
    bp = bp.strip()
    mi = 0
    ma = 7
    for l in bp[-3:-1]:
        if l == 'L':
            ma = mi + (ma - mi) // 2
        else:  # l == 'R'
            mi = mi + (ma - mi) // 2 + 1
    if bp[-1] == 'L':
        return mi
    else:  # bp[7] == 'R':
        return ma


def board_id(bp):
    return compute_id(find_row(bp), find_col(bp))


def test():
    t1 = {'bp': "BFFFBBFRRR", 'row': 70, 'col': 7, 'ID': 567}
    t2 = {'bp': "FFFBBBFRRR", 'row': 14, 'col': 7, 'ID': 119}
    t3 = {'bp': "BBFFBBFRLL", 'row': 102, 'col': 4, 'ID': 820}
    for t in [t1, t2, t3]:
        assert find_row(t['bp']) == t['row']
        assert find_col(t['bp']) == t['col']
        assert board_id(t['bp']) == t['ID']


def main():
    test()
    # Read input
    f = open("./input")
    # print(max(map(board_id, f)))
    # print(max(board_id(line) for line in f))
    ids = {board_id(line) for line in f}
    print(set(range(min(ids), max(ids) + 1)) - ids)


main()
