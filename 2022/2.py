from common import get_puzzle


SHAPE_TO_SCORE = {
    'X': 1,
    'Y': 2,
    'Z': 3,
}

RESULT_TO_SCORE = {
    'fail': 0,
    'draw': 3,
    'win': 6,
}


class Game:
    def __init__(self, puzzle: str):
        self.rounds = puzzle.split('\n')
        self.total_score = 0

    def round(self, line: str) -> int:
        score = 0
        match line.split(' '):
            case ['A', 'X']: score = SHAPE_TO_SCORE['X'] + RESULT_TO_SCORE['draw']
            case ['A', 'Y']: score = SHAPE_TO_SCORE['Y'] + RESULT_TO_SCORE['win']
            case ['A', 'Z']: score = SHAPE_TO_SCORE['Z'] + RESULT_TO_SCORE['fail']
            case ['B', 'X']: score = SHAPE_TO_SCORE['X'] + RESULT_TO_SCORE['fail']
            case ['B', 'Y']: score = SHAPE_TO_SCORE['Y'] + RESULT_TO_SCORE['draw']
            case ['B', 'Z']: score = SHAPE_TO_SCORE['Z'] + RESULT_TO_SCORE['win']
            case ['C', 'X']: score = SHAPE_TO_SCORE['X'] + RESULT_TO_SCORE['win']
            case ['C', 'Y']: score = SHAPE_TO_SCORE['Y'] + RESULT_TO_SCORE['fail']
            case ['C', 'Z']: score = SHAPE_TO_SCORE['Z'] + RESULT_TO_SCORE['draw']

        return score

    def round_2(self, line: str) -> int:
        score = 0
        match line.split(' '):
            case ['A', 'X']: score = SHAPE_TO_SCORE['Z'] + RESULT_TO_SCORE['fail']
            case ['A', 'Y']: score = SHAPE_TO_SCORE['X'] + RESULT_TO_SCORE['draw']
            case ['A', 'Z']: score = SHAPE_TO_SCORE['Y'] + RESULT_TO_SCORE['win']
            case ['B', 'X']: score = SHAPE_TO_SCORE['X'] + RESULT_TO_SCORE['fail']
            case ['B', 'Y']: score = SHAPE_TO_SCORE['Y'] + RESULT_TO_SCORE['draw']
            case ['B', 'Z']: score = SHAPE_TO_SCORE['Z'] + RESULT_TO_SCORE['win']
            case ['C', 'X']: score = SHAPE_TO_SCORE['Y'] + RESULT_TO_SCORE['fail']
            case ['C', 'Y']: score = SHAPE_TO_SCORE['Z'] + RESULT_TO_SCORE['draw']
            case ['C', 'Z']: score = SHAPE_TO_SCORE['X'] + RESULT_TO_SCORE['win']

        return score

    def run(self):
        for r in self.rounds:
            self.total_score += self.round(r)

    def run_2(self):
        for r in self.rounds:
            self.total_score += self.round_2(r)


def answer_1(puzzle) -> int:
    game = Game(puzzle)
    game.run()

    return game.total_score


def answer_2(puzzle) -> int:
    game = Game(puzzle)
    game.run_2()

    return game.total_score


if __name__ == "__main__":
    puzzle = get_puzzle("2_input.txt")

    print(f"Answer 1: {answer_1(puzzle)}")
    print(f"Answer 2: {answer_2(puzzle)}")
