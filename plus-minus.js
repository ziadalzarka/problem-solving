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
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

/**
 * Calculate ratios
 * @date 2022-03-26
 * @param {number[]} arr
 * @returns {any}
 */
function plusMinus(arr) {
  const counts = {
    positive: 0,
    negative: 0,
    zeros: 0,
  };

  arr.forEach((item) => {
    if (item > 0) {
      counts.positive++;
    } else if (item < 0) {
      counts.negative++;
    } else {
      counts.zeros++;
    }
  });

  const total = counts.negative + counts.positive + counts.zeros;

  return [
    counts.positive / total,
    counts.negative / total,
    counts.zeros / total,
  ];
}

function main() {
  const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

  const n = parseInt(readLine().trim(), 10);

  const arr = readLine()
    .replace(/\s+$/g, "")
    .split(" ")
    .map((arrTemp) => parseInt(arrTemp, 10));

  const result = plusMinus(arr);

  ws.write(result[0] + "\n");
  ws.write(result[1] + "\n");
  ws.write(result[2] + "\n");

  ws.end();
}
