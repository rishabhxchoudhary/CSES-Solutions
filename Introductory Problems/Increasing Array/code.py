n = int(input())
l = list(map(int,input().split()))
_max = l[0]
ans = 0
for i in range(n):
    ans += max(0, _max-l[i])
    _max = max(_max,l[i])
print(ans)