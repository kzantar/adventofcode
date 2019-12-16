import re
from collections import defaultdict
from itertools import chain

regxp = re.compile(r'Step (\w) must be finished before step (\w) can begin.')


steps = defaultdict(list)

for line in open('07_input.txt'):
    (first, second), *_ = regxp.findall(line)
    steps[first].append(second)


def can(step):
    for relation in steps.values():
        if step in relation:
            return False

    return True


def delete(step):
    for relation in steps.values():
        if step in relation:
            relation.remove(cur_step)

    del steps[step]


result = []
waits = set(chain.from_iterable(steps.values()))
possible = set(steps.keys()) - waits
while possible:
    cur_step = min(step for step in possible if can(step))
    result.append(cur_step)
    possible.discard(cur_step)
    possible |= set(steps[cur_step])
    delete(cur_step)


print('Result:', ''.join(result))

# cur_step = None
# for k, v in steps:
#     if v == []:
#         cur_step = k

# result = [cur_step]
# availables = set()
# for k, v in steps:
#     if cur_step in v:
