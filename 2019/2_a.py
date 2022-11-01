def add(pos, stack):
    stack[stack[pos+3]] = stack[stack[pos+1]] + stack[stack[pos+2]]


def mul(pos, stack):
    stack[stack[pos+3]] = stack[stack[pos+1]] * stack[stack[pos+2]]


def solve(stack):
    opcodes = {
        1: add,
        2: mul,
    }

    cur = 0
    while stack[cur] != 99:
        func = opcodes.get(stack[cur])
        func(cur, stack)
        cur += 4

    return stack


assert solve([1,0,0,0,99]) == [2,0,0,0,99]
assert solve([2,3,0,3,99]) == [2,3,0,6,99]
assert solve([2,4,4,5,99,0]) == [2,4,4,5,99,9801]
assert solve([1,1,1,4,99,5,6,0,99]) == [30,1,1,4,2,5,6,0,99]


if __name__ == '__main__':
    data = open('data/2.txt').read()
    stack = [int(num) for num in data.split(',')]

    stack[1] = 12
    stack[2] = 2

    solve(stack)
    print(stack[0])