const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

let inputLines = [];

rl.on('line', function (line) {
    inputLines.push(line.trim());
    if (inputLines.length === 2) {
        const n = parseInt(inputLines[0]);
        const numbers = inputLines[1].split(' ').map(Number);
        const actualSum = numbers.reduce((acc, val) => acc + val, 0);
        const expectedSum = n * (n + 1) / 2;
        console.log(expectedSum - actualSum);
        rl.close();
    }
});
