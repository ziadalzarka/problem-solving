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
 * Complete the 'climbingLeaderboard' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY ranked
 *  2. INTEGER_ARRAY player
 */

/**
 * Removes duplicates from leaderboard
 * @date 2022-03-27
 * @param {number[]} leaderboard
 * @returns {void}
 */
function deduplicate(leaderboard) {
  for (let i = 0; i < leaderboard.length; i++) {
    if (leaderboard[i] === leaderboard[i + 1]) {
      leaderboard.splice(i--, 1);
    }
  }
}

// 100 100 50 40 40 20 10
// 5 25 50

/**
 * Find the index of the exact score or the score right higher than it
 * @date 2022-03-27
 * @param {number[]} leaderboard
 * @param {number} score
 * @param {number} start
 * @param {number} end
 * @returns {number}
 */
const findPositionOnLeaderboard = function (leaderboard, score, start, end) {
  // Base Condition
  if (start > end) {
    return start + 1;
  }

  // Find the middle index
  let mid = Math.floor((start + end) / 2);

  // Compare mid with given key x
  if (leaderboard[mid] === score) return mid + 1;

  // If element at mid is greater than x,
  // search in the left half of mid
  if (leaderboard[mid] < score)
    return findPositionOnLeaderboard(leaderboard, score, start, mid - 1);
  // If element at mid is smaller than x,
  // search in the right half of mid
  else return findPositionOnLeaderboard(leaderboard, score, mid + 1, end);
};

/**
 * Calculate positions on the leader board
 * @date 2022-03-27
 * @param {number[]} leaderboard
 * @param {number[]} player
 * @returns {number[]}
 */
function climbingLeaderboard(leaderboard, player) {
  deduplicate(leaderboard);

  const end = leaderboard.length - 1;

  return player.map((score) =>
    findPositionOnLeaderboard(leaderboard, score, 0, end)
  );
}

function main() {
  const ws = fs.createWriteStream(process.env.OUTPUT_PATH);

  const rankedCount = parseInt(readLine().trim(), 10);

  const ranked = readLine()
    .replace(/\s+$/g, "")
    .split(" ")
    .map((rankedTemp) => parseInt(rankedTemp, 10));

  const playerCount = parseInt(readLine().trim(), 10);

  const player = readLine()
    .replace(/\s+$/g, "")
    .split(" ")
    .map((playerTemp) => parseInt(playerTemp, 10));

  const result = climbingLeaderboard(ranked, player);

  ws.write(result.join("\n") + "\n");

  ws.end();
}
