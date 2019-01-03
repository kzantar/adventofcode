import re
from bitarray import bitarray


state_pattern = re.compile(r'initial state: (.+)')
rule_pattern = re.compile(r'(.{5}) => (.)')
scaler = 5
generations = 10000

class Garden:
    def __init__(self):
        self.state = bitarray()
        self.rules = {}
        self.start = 0

    def __repr__(self):
        return ''.join('#' if b else '.' for b in self.state)

    def load(self, filename):
        lines = open(filename).readlines()
        line = state_pattern.match(lines[0]).group(1)
        self.state = self.scale(bitarray(line.replace('#', '1').replace('.', '0')))

        for line in lines[2:]:
            rule, goal = rule_pattern.match(line).groups()
            rule = rule.replace('#', '1').replace('.', '0')
            goal = goal.replace('#', '1').replace('.', '0')
            self.rules[rule] = int(goal)

    def scale(self, state: bitarray) -> bitarray:
        if 1 in state[:3]:
            state = bitarray('0' * scaler) + state
            self.start -= scaler

        if 1 in state[-3:]:
            state.extend('0' * scaler)

        first_plant = state.index('1')
        if first_plant > 10:     
            new_start = first_plant - scaler
            state = state[new_start:]
            self.start += new_start

        return state

    def step(self):
        state = bitarray()
        for idx in range(self.state.length()):
            sub = self.state[idx-2:idx+3]
            if len(sub) != 5:
                state.append(0)
                continue

            state.append(self.rules.get(sub.to01(), 0))

        self.state = self.scale(state)
        # self.state = state

    @property
    def score(self):
        result = 0
        for i, pot in enumerate(self.state):
            if pot:
                result += i + self.start

        return result


if __name__ == '__main__':
    g = Garden()
    g.load('12_input.txt')

    # print(f'{0:3}: {g}')
    for iteration in range(1, generations + 1):
        g.step()
        # print(f'{iteration:3}: {g}')


    print(f'Score: {g.score}')