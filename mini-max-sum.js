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
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

/**
 * Calculate min and max of 4 numbers sum
 * @date 2022-03-27
 * @param {number[]} arr
 * @returns {number[]} min, max
 */
function miniMaxSum(arr) {
  let min = arr[0];
  let max = arr[0];

  let minSum = 0;
  let maxSum = 0;

  let numbersSummed = 0;

  arr.forEach((num) => {
    if (num > max) {
      max = num;
    }

    if (num < min) {
      min = num;
    }

    if (numbersSummed < 4) {
      minSum += num;
      maxSum += num;
      numbersSummed++;
    } else if (max === num) {
      maxSum -= min;
      maxSum += max;
    } else if (min === num) {
      minSum -= max;
      minSum += min;
    } else {
      minSum -= max;
      maxSum -= min;
      minSum += num;
      maxSum += num;
    }
  });

  return [minSum, maxSum];
}

function main() {
  const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

  const arr = readLine()
    .replace(/\s+$/g, "")
    .split(" ")
    .map((arrTemp) => parseInt(arrTemp, 10));

  const [min, max] = miniMaxSum(arr);

  ws.write(`${min} ${max}` + "\n");

  ws.end();
}
