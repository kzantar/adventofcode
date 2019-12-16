from collections import defaultdict
from itertools import cycle
import re
from collections import deque
from array import array

debug = True

pattern = re.compile(r'(\d+) players; last marble is worth (\d+) points')


class Circle:
    def __inoit__(self, players, last_marble):
        self.board = {0: 0}
        self.players = players
        self.last_marble = last_marble
        self.score = defaultdict(list)

    def run(self):
        if debug:
            print(f'[-] {self.board.values()}')
        current = 0
        player = 1
        place = 0

        while current < self.last_marble:
            place = self.step(player, current, place)
            player += 1
            if player > self.players:
                player %= self.players
            current += 1

        winner, score = self.get_winner()
        print(f'Winner is #{winner} player. Score: {score}')
        return score

    def step(self, player, current, place):
        number = current + 1
        if number % 23 == 0:
            self.score[player].append(number)
            place = self.incr(place, -7)
            self.score[player].append(self.board.pop(place))
            if debug:
                print(f'[{player}] {self.board.values()}')
            return place

        index = self.incr(place, 2)
        self.board[index] = number

        if debug:
            print(f'[{player}] {self.board.values()}')

        return index

    def incr(self, var, value):
        var += value
        length = len(self.board)
        if var > length:
            var -= length
        elif var < 0:
            var += length

        return var
        

    def get_winner(self):
        player, score = max(
            self.score.items(), 
            key=lambda x: sum(x[1])
        )

        return player, sum(score)


if __name__ == '__main__':
    # for players, last_marble, score in [
    #     (9, 25, 32),
    #     (10, 1618, 8317),
    #     (13, 7999, 146373),
    #     (17, 1104, 2764),
    #     (21, 6111, 54718),
    #     (30, 5807, 37305),
    # ]:
    #     game = Circle(players, last_marble)
    #     assert game.run() == score, f'players: {players}, marbles: {last_marble}'

    # players, last_marble = pattern.match(open('09_input.txt').read()).groups()
    game = Circle(419, 710520)
    game.run()


