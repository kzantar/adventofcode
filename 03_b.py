import re
import numpy as np
from collections import namedtuple

Rectangle = namedtuple('Rectangle', 'id x1 y1 x2 y2') 

# #id @ left,top: width x height
claim_pattern = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')

# claims_input = [
#     '#1 @ 1,3: 4x4',
#     '#2 @ 3,1: 4x4',
#     '#3 @ 5,5: 2x2',
# ]

claims_input = open('03_input.txt')

def claim_to_rect(claim):
    id, left, top, width, height = claim
    x1 = left
    y1 = top
    x2 = left + width
    y2 = top + height
    return Rectangle(id, x1, y1, x2, y2)


def intersect(rect_1, rect_2):
    x1 = max(rect_1.x1, rect_2.x1)
    y1 = max(rect_1.y1, rect_2.y1)
    x2 = min(rect_1.x2, rect_2.x2)
    y2 = min(rect_1.y2, rect_2.y2)

    if x1 >= x2 or y1 >= y2:
        return False

    return True


rects = []
for line in claims_input:
    claim = claim_pattern.match(line).groups()
    claim = [int(v) for v in claim]
    rects.append(claim_to_rect(claim))

n = len(rects)

for i, rect in enumerate(rects):
    others = rects[:]
    others.pop(i)
    if all(not intersect(rect, r) for r in others):
        print('Result:', rect.id)
        break
