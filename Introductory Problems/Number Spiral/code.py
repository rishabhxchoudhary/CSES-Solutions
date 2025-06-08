for _ in range(int(input())):
    row, col = map(int, input().split())

    if row>=col:
        if row & 1:
            print((row - 1) * (row - 1) + col)
        else:
            print((row) * (row) - col + 1)
    else:
        if col & 1:
            print((col) * (col) - row + 1)
        else:
            print((col - 1) * (col - 1) + row)
