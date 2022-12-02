from pathlib import Path

PUZZLE_DIR = Path("./puzzle")


def get_puzzle(filename: str) -> str:
    return PUZZLE_DIR.joinpath(filename).read_text().strip()
