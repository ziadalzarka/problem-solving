/**
 * @param {number[]} nums
 * @return {number}
 */
var longestConsecutive = function (nums) {
  if (nums.length == 0) {
    return 0;
  }

  if (nums.length == 1) {
    return 1;
  }

  const hashmap = new Set();

  for (const num of nums) {
    hashmap.add(num);
  }

  let longestLength = 1;
  let currentLength = 1;

  for (const value of hashmap.values()) {
    if (hashmap.has(value + 1)) {
      continue;
    }

    while (hashmap.has(value - currentLength)) {
      currentLength++;
    }

    if (currentLength > longestLength) {
      longestLength = currentLength;
    }

    currentLength = 1;
  }

  return longestLength;
};

console.log(
  longestConsecutive([
    -7, -1, 3, -9, -4, 7, -3, 2, 4, 9, 4, -9, 8, -7, 5, -1, -7,
  ]),
);
