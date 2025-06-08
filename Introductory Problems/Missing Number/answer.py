n = int(input())
_sum = sum(map(int,input().split()))
print(n*(n+1)//2 - _sum)