import re

input_data = open('05_input.txt').read().strip()


import string

pairs = []
for ch in string.ascii_lowercase:
    pairs.append('{}{}'.format(ch, ch.upper()))
    pairs.append('{}{}'.format(ch.upper(), ch))


def react(polymer):
    found = False
    for pair in pairs:
        if pair in polymer:
            found = True
            polymer = polymer.replace(pair, '')

    if found:
        result = react(polymer)            
    else:
        result = len(polymer)

    return result


results = []
for ch in string.ascii_lowercase:
    polymer = re.sub(r'{}|{}'.format(ch, ch.upper()), '', input_data)
    results.append(react(polymer))

print('Result:', min(results))