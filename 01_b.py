from itertools import cycle

frequency = 0
history = [frequency]
changes = tuple(open('01_input.txt'))
for change in cycle(changes):
    frequency += int(change)
    if frequency in history:
        print('Result:', frequency)
        break
    print(frequency)
    history.append(frequency)

print('Not found repeat frequency')