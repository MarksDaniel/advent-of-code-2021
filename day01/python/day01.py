def solve(depths, window):
    return sum(x < y for (x, y) in zip(depths, depths[window:]))

with open("../input.txt")as file:
    depths = [int(n) for n in file.readlines()] 
    print(solve(depths, 1)); # Part One 
    print(solve(depths, 3)); # Part Two 
