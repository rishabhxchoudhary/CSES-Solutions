const fs = require('fs');
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
let n = parseInt(input[0], 10);
let ans = 0;
let p = 5;
while (n) {
    n = parseInt(n / p, 10);
    ans += n;
}
console.log(ans);