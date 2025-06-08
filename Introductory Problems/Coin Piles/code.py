for _ in range(int(input())):
    a,b = map(int,input().split())
    if a > 2 * b or b > 2*a:
        print("NO")
        continue
    a = a%3
    b = b%3
    if (a==1 and b==2) or (a==2 and b==1) or (a==0 and b==0):
        print("YES")
    else:
        print("NO")