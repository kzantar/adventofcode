
frequency = 0
for change in open('01_input.txt'):
    frequency += eval(change)

print('Result:', frequency)