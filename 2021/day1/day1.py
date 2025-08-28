def puzzle1(numbers):
    count = 0
    for i in range(1,len(numbers)):
        if numbers[i] > numbers[i-1]:
            count +=1
    print(count)


def puzzle2(numbers):
    count = 0
    for i in range(3, len(numbers)):
        if sum(numbers[i-2:i+1]) > sum(numbers[i-3:i]):
            count += 1
    print(count)


def main():
    # Read input
    f = open("/home/linaewen/PycharmProjects/advent2021/day1/input")
    numbers = list(map(int, f.readlines()))
    # numbers = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263]

    puzzle1(numbers)
    puzzle2(numbers)


main()
