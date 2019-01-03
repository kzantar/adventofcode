import re
from collections import defaultdict
from itertools import chain

from tabulate import tabulate


WORKERS = 5
STEP_TIMEOUT = 60

regxp = re.compile(r'Step (\w) must be finished before step (\w) can begin.')


class ToDo:
    def __init__(self, filename):
        self._filename = filename
        self._possible = set()
        self._steps = defaultdict(list)

        self.load()

    def load(self):
        for line in open(self._filename):
            (first, second), *_ = regxp.findall(line)
            self._steps[first].append(second)

        waits = set(chain.from_iterable(self._steps.values()))
        self._possible = set(self._steps.keys()) - waits

    def get(self):
        if not self._possible:
            return None

        step = min((step for step in self._possible if self.can(step)), default=None)
        if step:
            self._possible.discard(step)
        return step

    def can(self, step):
        for relation in self._steps.values():
            if step in relation:
                return False

        return True

    def step_finish(self, step):
        self._possible |= set(self._steps[step])

        for relation in self._steps.values():
            if step in relation:
                relation.remove(step)

        del self._steps[step]

    def done(self):
        return not self._steps


class Worker:
    def __init__(self, todo, dst):
        self._todo = todo
        self._dst = dst
        self._step = None
        self._timer = None
        self._required_time = None

    def next(self):
        self._step = self._todo.get()
        self._timer = 0
        self._required_time = self.get_time()

    def finish(self):
        self._dst.append(self._step)
        self._todo.step_finish(self._step)
        self._step = None

    def get_time(self):
        if self._step is None:
            return

        return STEP_TIMEOUT + ord(self._step) - ord('A') + 1

    def tick(self):
        if self._step is None:
            self.next()

        if self._step is None:
            return

        if self._timer >= self._required_time:
            self.finish()
            self.next()

        self._timer += 1

    def working(self):
        if self._timer is None or self._required_time is None:
            return False

        return self._timer <= self._required_time

def stats(cnt, workers, result):
    chunks = []
    chunks.append(f'{cnt:^7}')
    for worker in workers:
        chunks.append('{:^9}'.format(worker._step or '.'))
    chunks.append(''.join(result))

    print('\t'.join(chunks))


if __name__ == '__main__':
    todo = ToDo('07_input.txt')
    result = []
    counter = 0
    workers = [Worker(todo, result) for _ in range(WORKERS)]
    print(f'Second', *[f'Worker {i}' for i in range(WORKERS)], 'Done', sep='\t')
    while True:
        for worker in sorted(workers, key=lambda x: x._step if x._step is not None else 'ZZ'):
            worker.tick()
        stats(counter, workers, result)
        
        if todo.done() and all(not w.working() for w in workers):
            break

        counter += 1


    print('Result:', ''.join(result))
    print('Counter:', counter)




