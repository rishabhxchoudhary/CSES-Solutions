n = int(input())
p = 5
ans = 0
while n:
    n = n//p
    ans += n
print(ans)