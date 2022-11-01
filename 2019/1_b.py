data = open('data/1.txt').readlines()

def solve():
    result = sum(fuel_need(i) for i in data)
    print(result)


def fuel_need(mass):
    fuel = int(mass) // 3 - 2
    if fuel <= 0:
        return 0

    return fuel + fuel_need(fuel)


assert fuel_need(14) == 2
assert fuel_need(1969) == 966
assert fuel_need(100756) == 50346

if __name__ == '__main__':
    solve()