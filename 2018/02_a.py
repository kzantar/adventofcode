from collections import Counter

ids = open('02_input.txt')
two = 0
three = 0

for i in ids:
    counts = Counter(i).values()
    if 2 in counts:
        two += 1
    if 3 in counts:
        three += 1

print('Result', two * three)