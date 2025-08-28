# Read input
f = open("./input")

numbers = list(map(int, f.readlines()))
l = len(numbers)

"""

for i in range(l):
    for j in range(i, l):
        if numbers[i] + numbers[j] == 2020:
            print(numbers[i]*numbers[j])
            exit(0)
"""

for i in range(l):
    for j in range(l):
        for k in range(l):
            if numbers[i] + numbers[j] + numbers[k] == 2020:
                print(numbers[i]*numbers[j]*numbers[k])
                exit(0)