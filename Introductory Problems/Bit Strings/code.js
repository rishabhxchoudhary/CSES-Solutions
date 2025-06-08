const fs = require('fs')
const input = fs.readFileSync(0, 'utf-8').trim().split('\n');
const _n = parseInt(input[0], 10)
const n = BigInt(_n)


function bin_pow(a, b, MOD = 1_00_00_00_007n) {
    let ans = 1n;
    while (b > 0n) {
        if (b&1n) {
            ans = (ans * a)%MOD;
        }
        a = (a*a) % MOD;
        b = b >> 1n;
    }
    return ans;
}

console.log(bin_pow(2n, n).toString());