import sys


class Intcode:
    def __init__(self, data):
        self._data = open(data).read()
        self.instructions = {
            1: self._add,
            2: self._mul,
        }
        self.init()

    def init(self):
        self._memory = [int(num) for num in self._data.split(',')]
        self._pointer = 0

    def input(self, noun, verb):
        self._memory[1] = noun
        self._memory[2] = verb

    def _add(self):
        param1_addr = self._memory[self._pointer + 1]
        param2_addr = self._memory[self._pointer + 2]
        result_addr = self._memory[self._pointer + 3]
        self._memory[result_addr] = self._memory[param1_addr] + self._memory[param2_addr]
        self._pointer += 4

    def _mul(self):
        param1_addr = self._memory[self._pointer + 1]
        param2_addr = self._memory[self._pointer + 2]
        result_addr = self._memory[self._pointer + 3]
        self._memory[result_addr] = self._memory[param1_addr] * self._memory[param2_addr]
        self._pointer += 4

    @property
    def pointer(self):
        return self._memory[self._pointer]

    @property
    def result(self):
        # Result is kept in address 0
        return self._memory[0]
    

    def run(self):
        while self.pointer != 99:
            instruction = self.instructions.get(self.pointer)
            instruction()

        return self.result


if __name__ == '__main__':
    comp = Intcode('data/2.txt')
    
    for noun in range(100):
        for verb in range(100):
            comp.init()
            comp.input(noun, verb)
            if comp.run() == 19690720:
                print(100 * noun + verb)
                sys.exit()
