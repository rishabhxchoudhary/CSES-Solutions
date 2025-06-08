const fs = require('fs');
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
const n = parseInt(input[0],10);
if ([2,3].includes(n)){
    console.log("NO SOLUTION")
    process.exit(0);
}
const result = []
for(let i = 2; i <= n; i += 2) {
    result.push(i);
}

for(let i = 1; i <= n; i += 2) {
    result.push(i);
}

console.log(result.join(' '));