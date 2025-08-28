import re

# Read input
f = open("./input")

passports = []
new = {}
for line in f:
    if line == '\n' and new:
        passports.append(new)
        new = {}
    else:
        d = line.split()
        for e in d:
            entry = e.split(':')
            new[entry[0]] = entry[1]
passports.append(new)

count = 0
for passport in passports:
    #if {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'} <= set(passport):
    #    count += 1
    if 'byr' in passport:
        if not passport['byr'].isdigit():
            continue
        if not 1920 <= int(passport['byr']) <= 2002:
            continue
    else:
        continue

    if 'iyr' in passport:
        if not passport['iyr'].isdigit():
            continue
        if not 2010 <= int(passport['iyr']) <= 2020:
            continue
    else:
        continue

    if 'eyr' in passport:
        if not passport['eyr'].isdigit():
            continue
        if not 2020 <= int(passport['eyr']) <= 2030:
            continue
    else:
        continue

    if 'hgt' in passport:
        g = re.fullmatch(r'([0-9]+)(cm|in)', passport['hgt'])
        if g:
            if g.group(2) == 'cm':
                if not 150 <= int(g.group(1)) <= 193:
                    continue
            if g.group(2) == 'in':
                if not 59 <= int(g.group(1)) <= 76:
                    continue
    else:
        continue

    if 'hcl' in passport:
        if not re.fullmatch(r'#[a-f0-9]{6}', passport['hcl']):
            continue
    else:
        continue

    if 'ecl' in passport:
        if not passport['ecl'] in {'amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'}:
            continue
    else:
        continue

    if 'pid' in passport:
        if not re.fullmatch(r'[0-9]{9}', passport['pid']):
            continue
    else:
        continue

    count += 1

print(count)