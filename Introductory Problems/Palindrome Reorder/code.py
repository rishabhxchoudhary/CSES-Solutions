from collections import Counter

s = input().strip()
c = Counter(s)
half = []
middle = ""

for ch in c: 
    cnt = c[ch]
    if cnt % 2 == 1:
        if middle:
            print("NO SOLUTION")
            break
        middle = ch
    half.append(ch * (cnt // 2))

else: # python syntax: only goes in else case if loop ends normally without break condition
    half_str = ''.join(half)
    print(half_str + middle + half_str[::-1])