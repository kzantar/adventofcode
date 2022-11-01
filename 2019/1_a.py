data = open('data/1.txt').readlines()

def solve():
    result = sum(fuel_need(i) for i in data)
    print(result)


def fuel_need(mass):
    return int(mass) // 3 - 2


assert fuel_need(12) == 2
assert fuel_need(14) == 2
assert fuel_need(1969) == 654
assert fuel_need(100756) == 33583

if __name__ == '__main__':
    solve()