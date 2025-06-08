const fs = require('fs')
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
const n = parseInt(input[0], 10)
const l = input[1].split(' ').map(Number);
let ans = 0, maxSoFar = l[0];

for (let i = 1; i < l.length; i++) {
    if (l[i] < maxSoFar) {
        ans += maxSoFar - l[i];
    } else {
        maxSoFar = l[i];
    }
}
console.log(ans);