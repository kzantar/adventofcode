import numpy as np
 

data = []
for line in open('02_input.txt'):
    data.append(
        [ord(ch) for ch in list(line.strip())]
    )

data = np.array(data)

for row in data:
    res = data[np.count_nonzero(data - row, axis=1) == 1]
    if len(res) == 1:
        commons = (row - res[0]) == 0
        print('Commons:', ''.join(map(chr, row[commons])))
        break
