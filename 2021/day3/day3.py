def read_input(f):
    exemple = ['00100', '11110', '10110', '10111', '10101', '01111', '00111', '11100', '10000', '11001', '00010', '01010']
    # return exemple
    return f.read().splitlines()


def puzzle1(report):
    gamma = ''
    epsilon = ''
    for bit in range(len(report[0])):
        nb0 = 0
        nb1 = 0
        for line in report:
            if line[bit] == '0':
                nb0 += 1
            else:
                nb1 += 1
        if nb0 > nb1:
            gamma += '0'
            epsilon += '1'
        else:
            gamma += '1'
            epsilon += '0'
    print(int(gamma, 2) * int(epsilon, 2))


def most_common_bit(l, pos):
    nb0 = 0
    nb1 = 0
    for line in l:
        if line[pos] == '0':
            nb0 += 1
        else:
            nb1 += 1
    # print(f'nb0: {nb0} ; nb1: {nb1}')
    if nb0 > nb1:
        return '0'
    else:
        return '1'


def least_common_bit(l, pos):
    nb0 = 0
    nb1 = 0
    for line in l:
        if line[pos] == '0':
            nb0 += 1
        else:
            nb1 += 1
    if nb0 > nb1:
        return '1'
    else:
        return '0'


def oxygen_generation_rating(report):
    new_list = report
    for bit in range(len(report[0])):
        # print(f'Bit {bit}: {new_list}')
        most_common = most_common_bit(new_list, bit)
        new_list = [line for line in new_list if line[bit] == most_common]
        if len(new_list) == 1:
            return new_list[0]


def co2_scrubber_rating(report):
    new_list = report
    for bit in range(len(report[0])):
        # print(f'Bit {bit}: {new_list}')
        least_common = least_common_bit(new_list, bit)
        new_list = [line for line in new_list if line[bit] == least_common]
        if len(new_list) == 1:
            return new_list[0]


def puzzle2(report):
    print(int(oxygen_generation_rating(report), 2) * int(co2_scrubber_rating(report), 2))


def main():
    # Read input
    f = open("/home/linaewen/PycharmProjects/advent2021/day3/input")
    report = read_input(f)
    #print(report)

    puzzle1(report)
    puzzle2(report)


main()
