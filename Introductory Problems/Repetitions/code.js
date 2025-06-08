const fs = require('fs');

const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
const s = input[0]

let ans = 1
let final_ans = 1
for(let i = 1; i < s.length; i++) {
    if (s[i] == s[i-1]) {
        ans ++;
    } else {
        ans = 1
    }
    final_ans = Math.max(final_ans, ans);
}
console.log(final_ans)