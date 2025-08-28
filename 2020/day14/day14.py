def apply_mask_to_values(mask, line, memory):
    address, value = line.split(' = ')
    address = int(address[4:-1])
    value = f'{int(value):036b}'

    result = ''
    for car_mask, car_value in zip(mask, value):
        if car_mask in ['1', '0']:
            result += car_mask
        else:
            result += car_value

    memory[address] = int(result, 2)


def get_addresses(address):
    nb_floatings = 2**address.count('X')
    len_str = len(f'{nb_floatings:b}')
    addresses = []
    for i in range(nb_floatings):
        new = ''
        val = list(f'{i:0{len_str}b}')
        #print(val, address)
        for car in address:
            if car == 'X':
                new += val.pop()
            else:
                new += car
        addresses.append(new)
    return addresses


def apply_mask_to_addresses(mask, line, memory):
    address, value = line.split(' = ')
    address = f'{int(address[4:-1]):036b}'
    value = int(value)

    result = ''
    for car_mask, car_address in zip(mask, address):
        if car_mask == '0':
            result += car_address
        elif car_mask == '1':
            result += car_mask
        else:
            result += 'X'

    addresses = get_addresses(result)
    for add in addresses:
        memory[int(add, 2)] = value


def main():
    # Read input
    f = open("./input")

    memory = {}
    for line in f:
        if line.startswith("mask = "):
            mask = line.split(" = ")[1].strip()

        else:
            #apply_mask_to_values(mask, line, memory)
            apply_mask_to_addresses(mask, line, memory)

    print(sum(memory.values()))


main()
