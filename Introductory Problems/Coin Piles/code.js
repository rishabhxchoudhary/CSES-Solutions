const fs = require('fs');
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
let n = parseInt(input[0], 10);

for(let i = 1; i<=n; i++ ){
    const line = input[i].split(' ').map(Number);
    let a = line[0], b = line[1];
    if (a>2*b || b>2*a) {
        console.log("NO"); continue;
    }
    a = a%3;
    b = b%3;
    if ((a==1 && b==2) || (a==2 && b==1) || (a==0 && b==0)) {
        console.log("YES");
    } else {
        console.log("NO");
    }
}