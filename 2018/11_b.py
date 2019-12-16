import numpy as np


def power(x, y, serial):
    rack_id = x + 10
    power = ((rack_id * y) + serial) * rack_id
    hundreds = int(str(power)[-3])

    return hundreds - 5


def build_grid(width, heigth, serial):
    grid = [
        [power(x, y, serial) for x in range(1, width + 1)]
        for y in range(1, heigth + 1)
    ]

    grid = np.array(grid)

    return grid


def solve(grid, size):
    powers = {}
    width, heigth = grid.shape

    for y in range(0, heigth - size + 1):
        for x in range(0, width - size + 1):
            sub = grid[y:y+size, x:x+size]
            powers[(x+1, y+1)] = sub.sum()

    coords, total_power = max(powers.items(), key=lambda pair: pair[1])

    return coords, total_power


if __name__ == '__main__':
    # grid = build_grid(300, 300, 18)
    # assert solve(grid, 16) == ((90, 269), 113)

    # grid = build_grid(300, 300, 42)
    # assert solve(grid, 12) == ((232, 251), 119)
    
    grid = build_grid(300, 300, 8199)
    powers = {}
    for size in range(1, 301):
        coords, total_power = solve(grid, size)
        powers[(size, *coords)] = total_power

    coords, total_power = max(powers.items(), key=lambda pair: pair[1])
    print(f'Cell: {coords} Power: {total_power}')
    # Cell: (18, 234, 272) Power: 119
