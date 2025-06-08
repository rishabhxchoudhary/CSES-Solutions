n = int(input())
s = n * (n+1) // 2
if s&1:
    print("NO")
    exit(0)
s = s//2

set1 = []
set2 = []

s1 = 0
for i in reversed(range(1, n+1)):
    if s1 + i <= s:
        set1.append(i)
        s1 += i
    else:
        set2.append(i)
        
print("YES")
print(len(set1))
print(*set1)
print(len(set2))
print(*set2)
