import numpy as np


def power(x, y, serial):
    rack_id = x + 10
    power = ((rack_id * y) + serial) * rack_id
    hundreds = int(str(power)[-3])

    return hundreds - 5


def solve(width, heigth, serial):
    grid = [
        [power(x, y, serial) for x in range(1, width + 1)]
        for y in range(1, heigth + 1)
    ]

    grid = np.array(grid)

    powers = {}
    for y in range(0, heigth - 2):
        for x in range(0, width - 2):
            sub = grid[y:y+3, x:x+3]
            powers[(x+1, y+1)] = sub.sum()

    coords, total_power = max(powers.items(), key=lambda pair: pair[1])

    return coords, total_power


if __name__ == '__main__':
    assert power(3, 5, 8) == 4
    assert power(122, 79, 57) == -5
    assert power(217, 196, 39) == 0
    assert power(101, 153, 71) == 4

    assert solve(300, 300, 18) == ((33, 45), 29)
    assert solve(300, 300, 42) == ((21, 61), 30)
    
    coords, total_power = solve(300, 300, 8199)
    print(f'Cell: {coords} Power: {total_power}')
