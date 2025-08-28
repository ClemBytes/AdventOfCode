import math as m


def find_next_bus(start, ids):
    mini = m.inf
    for bus in ids:
        if bus - (start % bus) < mini:
            mini = bus - (start % bus)
            mini_id = bus
        #print(f"bus n° {bus}, mini: {mini}, mini_id: {mini_id}, bus - (start % bus): {bus - (start % bus)}")
    return mini_id * mini


def main():
    # Read input
    f = open("./input")
    start = int(f.readline())
    buses = f.readline().split(',')
    ids = [int(i) for i in buses if i.isdigit()]

    print(find_next_bus(start, ids))


main()
