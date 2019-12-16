import re


pattern = re.compile(
    r'position=<([-\d\s]+),([-\d\s]+)> velocity=<([-\d\s]+),([-\d\s]+)>'
)


class Point:
    def __init__(self, x, y, dx, dy):
        self.x = x
        self.y = y
        self.dx = dx
        self.dy = dy

    def __repr__(self):
        return f'<Point({self.x}, {self.y}, {self.dx}, {self.dy})>'

    def step(self):
        self.x += self.dx
        self.y += self.dy

    def backstep(self):
        self.x -= self.dx
        self.y -= self.dy


class Board:
    def __init__(self):
        self.points = []

    def load(self, filename):
        for line in open(filename):
            match = pattern.match(line)
            if match:
                x, y, dx, dy = map(int, match.groups())
                self.points.append(Point(x, y, dx, dy))

    def step(self):
        for point in self.points:
            point.step()

    def backstep(self):
        for point in self.points:
            point.backstep()

    def draw(self):
        min_x, max_x, min_y, max_y = self.get_rect()

        grid = []
        for y in range(min_y, max_y + 1):
            line = []
            for x in range(min_x, max_x + 1):
                if self.in_points(x, y):
                    line.append('#')
                else:
                    line.append('.')
            grid.append(''.join(line))

        print(f'Area: {self.area}')
        print('\n'.join(grid))
        print()

    def get_rect(self):
        min_x = min(self.points, key=lambda p: p.x).x
        max_x = max(self.points, key=lambda p: p.x).x
        min_y = min(self.points, key=lambda p: p.y).y
        max_y = max(self.points, key=lambda p: p.y).y

        return min_x, max_x, min_y, max_y

    @property
    def area(self):
        x1, x2, y1, y2 = self.get_rect()

        return (x2 - x1) * (y2 - y1)

    def in_points(self, x, y):
        for point in self.points:
            if point.x == x and point.y == y:
                return True
        return False


if __name__ == '__main__':
    b = Board()
    b.load('10_input.txt')
    prev_area = float('inf')
    timer = 0
    while b.area <= prev_area:
        prev_area = b.area
        b.step()
        timer += 1
    b.backstep()
    b.draw()
    print(f'Timer: {timer - 1}')
    