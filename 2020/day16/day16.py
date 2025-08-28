def read_rules(r):
    rules = {}
    for line in r:
        name, info = line.strip().split(': ')
        ra1, ra2 = info.split(' or ')
        ra1a, ra1b = list(list(map(int, ra1.split('-'))))
        ra2a, ra2b = list(list(map(int, ra2.split('-'))))
        rules[name] = set(range(ra1a, ra1b + 1)) | set(range(ra2a, ra2b + 1))
    return rules


def puzzle1(rules, nearby):
    s = 0
    for ticket in nearby:
        for nb in ticket:
            if all(nb not in r for r in rules.values()):
                s += nb
    print(s)


def discard_invalids(rules, nearby):
    new = []
    for ticket in nearby:
        valid = all(
            any(nb in r for r in rules.values())
            for nb in ticket
        )
        if valid:
            new.append(ticket)
    return new


def puzzle2(my_ticket, valid, rules):
    #TODO
    return 0


def main():
    # Read input
    my_ticket = [79, 193, 53, 97, 137, 179, 131, 73, 191, 139, 197, 181, 67, 71, 211, 199, 167, 61, 59, 127]
    my_ticket_simp = [11, 12, 13]

    r = open("./rules_simp")
    rules = read_rules(r)

    n = open("./nearby_simp")
    nearby = [list(map(int, line.strip().split(','))) for line in n]

    valid = discard_invalids(rules, nearby) # + my_ticket
    print(valid)

    puzzle1(rules, nearby)


main()
