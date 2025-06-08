a = input().strip()
n = len(a)
ans = 1
final_ans = 1
for i in range(1,n):
    if a[i-1] == a[i]:
        ans += 1
    else:
        ans = 1
    final_ans = max(final_ans, ans)
print(final_ans)
    