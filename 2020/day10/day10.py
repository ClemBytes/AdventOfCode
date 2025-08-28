def first(l):
    ones = 0
    trees = 0
    for i in range(1, len(l)):
        if l[i] - l[i-1] == 1:
            ones += 1
        elif l[i] - l[i-1] == 3:
            trees += 1

    return ones * trees


values = {}
def second(l, i):
    if i < 0:
        return 0
    elif i == 0:
        return 1
    elif i in values:
        return values[i]
    elif i not in l:
        return 0
    else:
        new = second(l, i - 1) + second(l, i - 2) + second(l, i - 3)
        values[i] = new
        return new

from functools import lru_cache


def memoize(f):
    cache = {}
    def g(*args):
        key = tuple(args)
        if key in cache:
            return cache[key]
        ret = f(*args)
        cache[key] = ret
        return ret
    return g


def second(l, i):
    @memoize
    def third(i):
        if i < 0:
            return 0
        elif i == 0:
            return 1
        elif i not in l:
            return 0
        else:
            return third(i - 1) + third(i - 2) + third(i - 3)
    return third(i)


def main():
    # Read input
    f = open("./input")
    l = [0]
    l += sorted(map(int, f))
    l.append(max(l) + 3)
    print(second(l, l[-1]))


main()