const fs = require('fs');
const s = fs.readFileSync(0, 'utf-8').trim().split('\n')[0];

const count = {}
for (ch of s) {
    count[ch] = (count[ch] || 0) + 1;
}

const half  = []
let middle = ""
let odd_count = 0

const keys = Object.keys(count);

for (ch of keys) {
    const cnt = count[ch];
    if (cnt&1) {
        odd_count += 1;
        if (odd_count > 1) {
            console.log("NO SOLUTION");
            process.exit(0);
        }
        middle = ch;
    }
    half.push(ch.repeat(Math.floor(cnt/2)));
}

const half_str = half.join("");
const result = half_str + middle + [...half_str].reverse().join('');
console.log(result)