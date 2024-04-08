/**
 * @param {number[]} numbers
 * @param {number} target
 * @return {number[]}
 */
var twoSum = function (numbers, target) {
  let startPointer = 0;
  let endPointer = numbers.length - 1;

  while (startPointer < endPointer) {
    const sum = numbers[startPointer] + numbers[endPointer];

    if (sum > target) {
      endPointer--;
    }

    if (sum < target) {
      startPointer++;
    }

    if (sum == target) {
      return [startPointer + 1, endPointer + 1];
    }
  }
};

console.log(twoSum([2, 7, 11, 15], 9));
