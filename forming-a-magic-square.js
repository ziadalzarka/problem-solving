"use strict";

const fs = require("fs");

process.stdin.resume();
process.stdin.setEncoding("utf-8");

let inputString = "";
let currentLine = 0;

process.stdin.on("data", function (inputStdin) {
  inputString += inputStdin;
});

process.stdin.on("end", function () {
  inputString = inputString.split("\n");

  main();
});

function readLine() {
  return inputString[currentLine++];
}

/*
 * Complete the 'formingMagicSquare' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY s as parameter.
 */

function findMagicNumber(n) {
  const max = Math.pow(n, 2);
  const sum = (max / 2) * (1 + max);

  return sum / n;
}

/**
 * Sum numbers
 * @date 2022-03-27
 * @param {number[]} row
 * @returns {number}
 */
function sum(row) {
  return row.reduce((p, c) => p + c, 0);
}

/**
 * Find cost of converting into a magic square
 * @date 2022-03-27
 * @param {number[][]} s
 * @returns {number}
 */
function formingMagicSquare(s) {
  const n = s.length;
  const magicNumber = findMagicNumber(n);

  let cost = 0;

  s.forEach((row) => {
    cost += Math.abs(sum(row) - magicNumber);
  });

  return cost;
}

function main() {
  const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

  let s = Array(3);

  for (let i = 0; i < 3; i++) {
    s[i] = readLine()
      .replace(/\s+$/g, "")
      .split(" ")
      .map((sTemp) => parseInt(sTemp, 10));
  }

  const result = formingMagicSquare(s);

  ws.write(result + "\n");

  ws.end();
}
