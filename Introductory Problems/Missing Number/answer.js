const fs = require('fs');

const input = fs.readFileSync(0, 'utf-8').trim().split('\n');

const n = parseInt(input[0]);
const nums = input[1].split(' ').map(Number);

const expectedSum = n * (n + 1) / 2;
const actualSum = nums.reduce((a, b) => a + b, 0);

console.log(expectedSum - actualSum);