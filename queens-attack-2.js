'use strict';

const fs = require('fs');

process.stdin.resume();
process.stdin.setEncoding('utf-8');

let inputString = '';
let currentLine = 0;

process.stdin.on('data', inputStdin => {
    inputString += inputStdin;
});

process.stdin.on('end', _ => {
    inputString = inputString.replace(/\s*$/, '')
        .split('\n')
        .map(str => str.replace(/\s*$/, ''));

    main();
});

function readLine() {
    return inputString[currentLine++];
}

// Complete the queensAttack function below.
function queensAttack(n, k, r_q, c_q, obstacles) {
    
    // convert obstacles into a more processing efficient data structure
    const obstaclesMap = {};
    obstacles.forEach(([y, x]) => obstaclesMap[x] = { ...obstaclesMap[x], [y]: true });
    
    // define possible directions for the queens
    // x is the number added to current position's x
    // y is the number added to current position's y
    const directions = [
        // right
        { x: 1, y: 0 },
        // left
        { x: -1, y: 0 },
        // up
        { x: 0, y: 1 },
        // down
        { x: 0, y: -1 },
        // up-right
        { x: 1, y: 1 },
        // up-left
        { x: -1, y: 1 },
        // down-right
        { x: 1, y: -1 },
        // down-left
        { x: -1, y: -1 },
    ];
    
    let possibleMoves = 0;
    
    for (let direction of directions) {
        let stop = false;
        
        let x = c_q + direction.x;
        let y = r_q + direction.y;
        
        while (!stop) {            
            // check if x is off board
            if (x <= 0 || x > n) {
                stop = true;
                continue;
            }
            
            // check if y is off board
            if (y <= 0 || y > n) {
                stop = true
                continue;
            }
            
            // check if position has an obstacle
            if (k > 0 && obstaclesMap[x] && obstaclesMap[x][y]) {
                stop = true;
                continue;
            }
            
            // count a possible move
            possibleMoves++;
            
            // generate new coordinates
            x += direction.x;
            y += direction.y;
        }
    }
    
    return possibleMoves;
}

function main() {
    const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

    const nk = readLine().split(' ');

    const n = parseInt(nk[0], 10);

    const k = parseInt(nk[1], 10);

    const r_qC_q = readLine().split(' ');

    const r_q = parseInt(r_qC_q[0], 10);

    const c_q = parseInt(r_qC_q[1], 10);

    let obstacles = Array(k);

    for (let i = 0; i < k; i++) {
        obstacles[i] = readLine().split(' ').map(obstaclesTemp => parseInt(obstaclesTemp, 10));
    }

    let result = queensAttack(n, k, r_q, c_q, obstacles);

    ws.write(result + "\n");

    ws.end();
}

