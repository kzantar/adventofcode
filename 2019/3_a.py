import numpy as np
import re
from scipy.spatial.distance import cityblock


SYMBOLS = {
    'L': '-',
    'R': '-',
    'U': '|',
    'D': '|', 
}


class Board:
    def __init__(self, height, width, center, scale=1):
        self.board = np.array(list('.' * height * width), dtype=str).reshape((height, width))
        self.center = center
        self.scale = scale
        self.cur = self.center
        self.point('o')

    def __str__(self):
        return '\n'.join(''.join(row) for row in self.board)

    __repr__ = __str__

    def point(self, num):
        if self.board[self.cur] == 'o':
            ch = 'o'
        elif self.board[self.cur] not in  f'.{num}':
            ch = 'X'
        else:
            ch = num
        
        self.board[self.cur] = ch

    def line(self, num, dest, length):
        dy = dx = 0
        if dest == 'L':
            dx = -1
        elif dest == 'R':
            dx = +1
        elif dest == 'U':
            dy = -1
        elif dest == 'D':
            dy = +1

        for i in range(1, length+1):
            self.cur = self.cur[0] + dy, self.cur[1] + dx
            self.point(num)

    def trace(self, num, path):
        self.cur = self.center

        for step in path.split(','):
            dest, length = re.match(r'(\w)(\d+)', step).groups()
            self.line(num, dest, int(length) // self.scale)

    def solve(self):
        distances = []
        w, h = self.board.shape
        for i in range(h):
            for j in range(w):
                if self.board[i, j] == 'X':
                    distances.append(
                        cityblock(self.center, (i, j))
                    )
        return min(distances)


if __name__ == '__main__':
    height = 30
    width = 30
    scale = 1
    center = height // 2, width // 2
    board = Board(height, width, center, scale)

    data = [
        'R75,D30,R83,U83,L12,D49,R71,U7,L72',
        'U62,R66,U55,R34,D71,R55,D58,R83',
    ]

    # data = [
    #     'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
    #     'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'
    # ]

    # data = [
    #     'R8,U5,L5,D5,R1,U2,L1,U2,R1,D1,L2',
    #     'U7,R6,D4,L4',
    # ]

    # data = open('data/3.txt').readlines()

    for num, line in enumerate(data, start=1):
        board.trace(num, line)

    print(board)
    print(board.solve())
