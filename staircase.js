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
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

function staircase(n) {
  let str = "";

  for (let i = 0; i < n; i++) {
    const spaces = n - (i + 1);
    const chars = i + 1;

    str += new Array(spaces).fill(" ").join("");
    str += new Array(chars).fill("#").join("");
    if (i < n - 1) {
      str += "\n";
    }
  }

  return str;
}

function main() {
  const ws = fs.createWriteStream(process.env.OUTPUT_PATH);
  const n = parseInt(readLine().trim(), 10);

  const result = staircase(n);
  ws.write(result + "\n");
  ws.end();
}
