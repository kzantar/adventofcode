import re
import numpy as np

fabric_side = 1000


# #id @ left,top: width x height
claim_pattern = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')

# claims_input = [
#     '#1 @ 1,3: 4x4',
#     '#2 @ 3,1: 4x4',
#     '#3 @ 5,5: 2x2',
# ]

claims_input = open('03_input.txt')

claims = []
for line in claims_input:
    claim = claim_pattern.match(line).groups()
    claim = [int(v) for v in claim]
    claims.append(claim)


fabric = np.zeros((fabric_side, fabric_side), dtype=int)

def overlap(square, label):
    square[square == 0] = label

for id, left, top, w, h in claims:
    if fabric[top:top+h, left:left+w].any():
        fabric[top:top+h, left:left+w] = np.where(fabric[top:top+h, left:left+w] == 0, id, -1)
    else:
        fabric[top:top+h, left:left+w] = id

print('Result:', (fabric == -1).sum())