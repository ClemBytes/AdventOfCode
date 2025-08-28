import re


def interpret_line(line):
    s = line.split(' bags contain ')
    parent = s[0]
    children = re.findall(r'([0-9]+) (.*?) bags?', s[1])
    return parent, children


def build_parents_tree(f):
    d = {}
    for line in f:
        parent, children = interpret_line(line)
        for count, name in children:
            #if name == 'clear gold':
            #    print(parent, children)
            if name in d:
                d[name].add(parent)
            else:
                d[name] = {parent}
    return d


def build_children_tree(f):
    d = {}
    for line in f:
        parent, children = interpret_line(line)
        d[parent] = set(children)
    return d


def main():
    # Read input
    f = open("./input")
    '''
    tree = build_parents_tree(f)

    colors = {"shiny gold"}
    l = len(colors)

    while True:
        for col in set(colors):
            colors |= tree.get(col, set())
        if l == len(colors):
            break
        l = len(colors)

    print(l-1)
    '''
    tree = build_children_tree(f)

    colors = {"shiny gold": 1}
    count = 0
    while True:
        next_colors = dict()
        for col, nb in colors.items():
            children = tree[col]
            for val, child in children:
                next_colors.setdefault(child, 0)
                next_colors[child] += int(val) * nb
        if colors == next_colors:
            break
        count += sum(next_colors.values())
        colors = next_colors

    print(count)

    #print(next_colors)
    #next_colors = {"shiny gold": 5, "faded blue": 18}





main()