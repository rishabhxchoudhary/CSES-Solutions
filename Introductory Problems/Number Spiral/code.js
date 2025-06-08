const fs = require('fs');
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');

const T = parseInt(input[0], 10);
let result = [];

for (let i = 1; i <= T; i++) {
    let [row, col] = input[i].split(' ').map(BigInt); // Use BigInt

    if (row >= col) {
        if (row % 2n === 1n) {
            result.push(((row - 1n) * (row - 1n) + col).toString());
        } else {
            result.push((row * row - col + 1n).toString());
        }
    } else {
        if (col % 2n === 1n) {
            result.push((col * col - row + 1n).toString());
        } else {
            result.push(((col - 1n) * (col - 1n) + row).toString());
        }
    }
}

console.log(result.join('\n'));
