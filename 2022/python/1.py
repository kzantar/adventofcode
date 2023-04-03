from common import get_puzzle


def get_sums(puzzle: str) -> list[int]:
    return [
        sum(
            calories
                for calories in map(int, elf_carrying.split())
        )
        for elf_carrying in puzzle.split('\n\n')
    ]


def answer_1(puzzle: str) -> int:
    return max(get_sums(puzzle))


def answer_2(puzzle: str) -> int:
    sums = get_sums(puzzle)
    return sum(
        s
        for s in sorted(sums, reverse=True)[:3]
    )


if __name__ == '__main__':
    puzzle = get_puzzle("1_input.txt")

    print(f"Answer 1: {answer_1(puzzle)}")
    print(f"Answer 2: {answer_2(puzzle)}")
