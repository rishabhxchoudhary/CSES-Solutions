const fs = require('fs');
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
const n = parseInt(input[0], 10);
const results = []
for(let i = 1; i<=n; i++ ){
    const n2 = i*i;
    const total = n2 * (n2-1) / 2;
    const do_attack = 4 * (i-1) * (i-2);
    results.push(total - do_attack);
}
console.log(results.join('\n'))