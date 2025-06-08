n = int(input())
for i in range(1,n+1):
    n2 = i*i
    total = n2 * (n2-1) // 2
    do_attack = 4 * (i-1) * (i-2)
    print(total - do_attack)