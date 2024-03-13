/**
 * @param {number[]} numbers
 * @param {number} target
 * @param {number} startingPoint
 * @return {number[]}
 */
var twoSum = function (numbers, target, startingPoint) {
  let startPointer = startingPoint || 0;
  let endPointer = numbers.length - 1;

  const solutions = [];

  while (startPointer < endPointer) {
    const sum = numbers[startPointer] + numbers[endPointer];

    if (sum > target) {
      endPointer--;
    }

    if (sum < target) {
      startPointer++;
    }

    if (sum == target) {
      solutions.push([numbers[startPointer], numbers[endPointer]]);
      startPointer++;
    }
  }

  return solutions;
};

/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var threeSum = function (nums) {
  const solutions = [];
  const map = {};
  const sorted = nums.sort((a, b) => a - b);
  const target = 0;

  for (let i = 0; i < sorted.length; i++) {
    const complement = target - sorted[i];

    const twoSumResult = twoSum(sorted, complement, i + 1);

    if (twoSumResult.length) {
      for (const twoSumSolution of twoSumResult) {
        const solution = [sorted[i], ...twoSumSolution];
        if (!map[solution]) {
          map[solution] = true;
          solutions.push(solution);
        }
      }
    }
  }

  return solutions;
};

console.log(threeSum([-2, 0, 1, 1, 2]));
