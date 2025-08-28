def read_input(kind):
    # "kind" must be "input" or "exemple"
    with open(f"/home/linaewen/PycharmProjects/advent2021/day10/{kind}") as f:
        chunks = [line.strip() for line in f]
        return chunks


def error_score(c):
    d = {
        ')': 3,
        ']': 57,
        '}': 1197,
        '>': 25137,
    }
    return d[c]


def match(opening, closing):
    if (opening == '(') and (closing == ')'):
        return True
    elif (opening == '[') and (closing == ']'):
        return True
    elif (opening == '{') and (closing == '}'):
        return True
    elif (opening == '<') and (closing == '>'):
        return True
    else:
        return False


def puzzle1(chunks):
    score = 0
    for line in chunks:
        pile = []
        for c in line:
            if c in ['(', '[', '{', '<']:
                pile.append(c)
            else:
                if match(pile[-1], c):
                    pile.pop()
                else:
                    score += error_score(c)
                    break
    return score


def complete_score(c):
    d = {
        ')': 1,
        ']': 2,
        '}': 3,
        '>': 4,
    }
    return d[c]


def find_closing(c):
    d = {
        '(': ')',
        '[': ']',
        '{': '}',
        '<': '>',
    }
    return d[c]


def puzzle2(chunks):
    scores = []
    for line in chunks:
        score = 0
        pile = []
        for c in line:
            if c in ['(', '[', '{', '<']:
                pile.append(c)
            else:
                if match(pile[-1], c):
                    pile.pop()
                else:
                    # Ignore corrupted lines
                    break
        else:
            while pile:
                opening = pile.pop()
                closing = find_closing(opening)
                score = (score * 5) + complete_score(closing)
            scores.append(score)

    scores = sorted(scores)
    return scores[len(scores)//2]


def main():
    # Read input
    chunks = read_input("input")
    #print(chunks)
    print(puzzle1(chunks))
    print(puzzle2(chunks))


main()
