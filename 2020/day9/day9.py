def is_ok(ind, l, limit=25):
    if ind <= limit:
        return True

    sub_list = l[ind - limit:ind]
    for i, nb1 in enumerate(sub_list):
        for nb2 in sub_list[i:]:
            if nb1 + nb2 == l[ind]:
                return True

    return False


def find_weakness(l):
    for i, val in enumerate(l):
        if not is_ok(i, l):
            return val


def find_sum(l, weakness):
    for istart in range(len(l)):
        s = 0
        istop = istart
        while s < weakness:
            s += l[istop]
            istop += 1

        if s == weakness:
            return min(l[istart:istop + 1]) + max(l[istart:istop + 1])


def main():
    # Read input
    f = open("./input")
    l = [int(i) for i in f]

    print(find_sum(l, find_weakness(l)))




main()