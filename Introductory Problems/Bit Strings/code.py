n = int(input())
M = 1_00_00_00_000 + 7

def bin_pow(a,b):
    ans = 1
    while b > 0:
        if b & 1:
            ans = (ans * a) % M
        a = (a * a) % M
        b = b >> 1
    return ans

print(bin_pow(2,n))