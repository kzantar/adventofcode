import re
from bitarray import bitarray


state_pattern = re.compile(r'initial state: (.+)')
rule_pattern = re.compile(r'(.{5}) => (.)')
scaler = 5
generations = 50000000000


class Garden:
    def __init__(self):
        self.state = ''
        self.rules = {}
        self.start = 0

    def __repr__(self):
        return self.state

    def load(self, filename):
        lines = open(filename).readlines()
        line = state_pattern.match(lines[0]).group(1)
        self.state = self.scale(line)

        for line in lines[2:]:
            rule, goal = rule_pattern.match(line).groups()
            self.rules[rule] = goal

    def scale(self, state: bitarray) -> bitarray:
        if '#' in state[:3]:
            state = ('.' * scaler) + state
            self.start -= scaler

        if '#' in state[-3:]:
            state = state +  ('.' * scaler)

        first_plant = state.index('#')
        if first_plant > 10:     
            new_start = first_plant - scaler
            state = state[new_start:]
            self.start += new_start

        return state

    def step(self):
        state = ''
        for idx in range(len(self.state)):
            sub = self.state[idx-2:idx+3]
            if len(sub) != 5:
                state += '.'
                continue

            if sub in self.rules:
                state += self.rules[sub]
            else:
                state += '.'

        self.state = self.scale(state)
        # self.state = state

    @property
    def score(self):
        result = 0
        for i, pot in enumerate(self.state):
            if pot == '#':
                result += i + self.start

        return result


if __name__ == '__main__':
    g = Garden()
    g.load('data/12_input.txt')

    # print(f'{0:3}: {g}')
    for iteration in range(1, generations + 1):
        g.step()

        if iteration % 100000 == 0:
            print(f'{iteration / generations:%}')
        # print(f'{iteration:3}: {g}')


    print(f'Score: {g.score}')