from enum import Enum
from collections import namedtuple
import re


class Cell:
    def __init__(self, value=None):
        if value is None:
            value = '.'
        self.value = value
    
    def __str__(self):
        return self.value

    def __repr__(self):
        return f'<Cell({self.value})>'
    
    def __eq__(self, other):
        return self.value == other.value

Cell.EMPTY = Cell('.')
Cell.START = Cell('o')
Cell.CROSS = Cell('X')


Point = namedtuple('Point', 'y x')


def distance(p1, p2):
    return abs(p1.y - p2.y) + abs(p1.x - p2.x)


class Board:
    def __init__(self, height, width):
        self.height = height
        self.width = width
        self.grid = [[Cell.EMPTY for x in range(self.width)] for y in range(self.height)]
        self.center = Point(self.height // 2, self.width // 2)
        self.grid[self.center.y][self.center.x] = Cell.START
        self.cur = self.center
    
    def __repr__(self):
        return self.__str__()
    
    def __str__(self):
        return '\n'.join(
            ''.join(c.value for c in row)
            for row in self.grid
        )

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
            self.cur = Point(self.cur.y + dy, self.cur.x + dx)
            self.point(num)
    
    def point(self, num):
        cell = self.grid[self.cur.y][self.cur.x]
        if cell == Cell.START:
            result =  Cell.START
        elif cell == Cell.EMPTY:
            result = Cell(num)
        elif cell != Cell(num):
            result = Cell.CROSS
        else:
            result = Cell(num)
        
        self.grid[self.cur.y][self.cur.x] = result

    def trace(self, num, path):
        self.cur = self.center

        for step in path.split(','):
            dest, length = re.match(r'(\w)(\d+)', step).groups()
            self.line(num, dest, int(length))
    
    def solve(self):
        distances = []
        for y in range(self.height):
            for x in range(self.width):
                if self.grid[y][x] == Cell.CROSS:
                    distances.append(
                        distance(self.center, Point(y, x))
                    )
        return min(distances)


if __name__ == "__main__":
    height = 100000
    width = 100000
    board = Board(height, width)

    # data = [
    #     'R8,U5,L5,D3',
    #     'U7,R6,D4,L4',
    # ]

    # data = [
    #     'R75,D30,R83,U83,L12,D49,R71,U7,L72',
    #     'U62,R66,U55,R34,D71,R55,D58,R83',
    # ]

    # data = [
    #     'R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51',
    #     'U98,R91,D20,R16,D67,R40,U7,R15,U6,R7'
    # ]

    # data = [
    #     'R8,U5,L5,D5,R1,U2,L1,U2,R1,D1,L2',
    #     'U7,R6,D4,L4',
    # ]

    data = open('data/3.txt').readlines()

    for num, line in enumerate(data, start=1):
        board.trace(str(num), line)

    # print(board)
    print(board.solve())