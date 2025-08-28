# Read input
f = open("./input")

lines = f.readlines()
l = len(lines)

# Interpret inputs
psswds = []
for line in lines:
    t = line.split()
    nbs = t[0].split('-')
    psswds.append({'mi': int(nbs[0]), 'ma': int(nbs[1]), 'le': t[1][0], 'code': t[2]})

count = 0

'''
for p in psswds:
    if p['code'].count(p['le']) >= p['mi'] and p['code'].count(p['le']) <= p['ma']:
        count += 1
'''

for p in psswds:
    s = 0
    if p['code'][p['mi']-1] == p['le'] :
        s += 1
    if p['code'][p['ma']-1] == p['le']:
        s += 1
    if s == 1:
        count += 1

print(count)