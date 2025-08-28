def read_input(f):
    program = []
    for line in f.readlines():
        l = line.split()
        instruction = l[0]
        value = int(l[1])
        program.append((instruction, value))
    return program


def run_program(program):
    visited = set()
    accumulator = 0
    current = 0
    cont = True
    while cont:
        visited.add(current)
        instruction, value = program[current]
        if instruction == "acc":
            accumulator += value
            current += 1
        elif instruction == "jmp":
            current += value
        elif instruction == "nop":
            current += 1
        else:
            raise ValueError(f"Unknown command: '{instruction}'")

        if current in visited:
            cont = False

    return accumulator


def main():
    # Read input
    f = open("./input")
    program = read_input(f)
    print(run_program(program))


main()