const fs = require('fs');
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
const n = parseInt(input[0], 10);

let sum = n * (n+1)/2;
if (sum&1) {
    console.log("NO");
    process.exit(0);
}
sum = sum /2;
let s1 = 0;
const set1 = []
const set2 = []

for(let i = n; i>=1; i--) {
    if (s1 + i <= sum) {
        set1.push(i);
        s1 += i;
    } else {
        set2.push(i);
    }
}
console.log("YES")
console.log(set1.length);
console.log(set1.join(' '))
console.log(set2.length);
console.log(set2.join(' '))