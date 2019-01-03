import numpy as np
from scipy.spatial.distance import cityblock
from termcolor import colored
from collections import Counter


SCALE_BOARD = 1
TOTAL_DISTANCE = 10000


class Point:
    def __init__(self, y, x, label='.'):
        self.y = y
        self.x = x
        self._label = label
        self.distances = {}

    def __repr__(self):
        return f'Point({self.y}, {self.x}, {self.label})'

    def __str__(self):
        return self.label

    def __eq__(self, other):
        return self.y == other.y and self.x == other.x

    def __lt__(self, other):
        return (self.y < other.y) and (self.x < other.x)

    def __gt__(self, other):
        return (self.y > other.y) and (self.x > other.x)

    def distance(self, point):
        return cityblock(
            [self.y, self.x],
            [point.y, point.x]
        )

    def calculate_distance(self, point):
        self.distances[point.label] = self.distance(point)

    @property
    def label(self):
        if not self.distances:
            return self._label

        if self._label != '.':
            return self._label
        
        if self.in_region:
            return '#'
        else:
            return '.'

    @property
    def in_region(self):
        return sum(self.distances.values()) < TOTAL_DISTANCE

    @label.setter
    def label(self, value):
        self._label = str(value)


class Board:
    def __init__(self):
        self.board = None
        self.points = []
        self.region = []

    def __repr__(self):
        return self.draw(self.board)
        
    def draw(self, board):
        result = []
        for row in board:
            for point in row:
                if point in self.points:
                    label = colored(point.label, 'red')
                else:
                    label = colored(point.label, 'white')
                result.append('{:^11}'.format(label))
            result.append('\n')

        return ''.join(result)


    def load(self, filename):
        coords = np.loadtxt(filename, dtype=int, delimiter=', ')
        # swap columns
        coords = coords[:, [1, 0]]
        y_max, x_max = coords.max(axis=0)
        self.board = np.array([
            [Point(y, x) for x in range(x_max+SCALE_BOARD)] 
            for y in range(y_max+SCALE_BOARD)
        ])
        for i, (y, x) in enumerate(coords, start=1):
            self.board[y, x].label = f'{i}'
            self.points.append(self.board[y, x])

    def fill(self):
        for place in self.board.flatten():
            for point in self.points:
                place.calculate_distance(point)

            if place.in_region:
                self.region.append(place)


if __name__ == '__main__':
    b = Board()
    b.load('06_input.txt')
    b.fill()

    print('Result:', len(b.region))

