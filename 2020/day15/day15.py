def play(starting_nbs, n=2020):
    last_time = {}

    for i in range(len(starting_nbs)):
        last_time[starting_nbs[i]] = i

    last = starting_nbs[-1]
    seen = set(starting_nbs[:-1])

    while i < n - 1:
        i += 1

        if last in seen:
            current = i-1 - last_time[last]
        else:
            current = 0

        seen.add(last)
        last_time[last] = i-1
        last = current

    return current


def test1():
    tests = {
        "0,3,6": 436,
        "1,3,2": 1,
        "2,1,3": 10,
        "1,2,3": 27,
        "2,3,1": 78,
        "3,2,1": 438,
        "3,1,2": 1836,
    }
    for t in tests:
        p = play(list(map(int, t.split(','))))
        if p == tests[t]:
            continue
        else:
            print(f"Problem on test : '{t}'. Should find '{tests[t]}' but find '{p}' instead.")


def test2():
    tests = {
        "0,3,6": 175594,
        "1,3,2": 2578,
        "2,1,3": 3544142,
        "1,2,3": 261214,
        "2,3,1": 6895259,
        "3,2,1": 18,
        "3,1,2": 362,
    }
    for t in tests:
        p = play(list(map(int, t.split(','))), 30000000)
        if p == tests[t]:
            continue
        else:
            print(f"Problem on test : '{t}'. Should find '{tests[t]}' but find '{p}' instead.")


def main():
    # Read input
    f = open("./input")
    starting_nbs = list(map(int, f.readline().split(',')))
    print(play(starting_nbs, 30000000))


main()
