n = int(input())
if n in [2,3]:
    print("NO SOLUTION")
    exit(0)
for i in range(2, n+1,2):
    print(i,end=" ")
for i in range(1,n+1, 2):
    print(i, end=" ")