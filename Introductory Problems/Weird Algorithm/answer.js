var readline = require('readline');

var r = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

r.on('line', function (line) {
    var vals = line.split(" ");
    let n = parseInt(vals[0], 10);

    let result = []

    while (n!=1) {
        result.push(n);
        if (n&1) {
            n = n*3+1;
        } else {
            n = n/2;
        }
    }
    result.push(1);

    console.log(result.join(" "));
});