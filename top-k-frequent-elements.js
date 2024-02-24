/**
 * @param {number[]} nums
 * @param {number} k
 * @return {number[]}
 */
var topKFrequent = function (nums, k) {
  let frequency = new Array(nums.length);
  const count = new Map();

  for (const num of nums) {
    count.set(num, (count.get(num) || 0) + 1);
  }

  count.forEach((value, key) => {
    if (frequency[value]) {
      frequency[value].push(key);
    } else {
      frequency[value] = [key];
    }
  });

  const result = [];

  for (let i = frequency.length - 1; i > -1; i--) {
    if (!frequency[i]) {
      continue;
    }

    for (const num of frequency[i]) {
      if (k == 0) {
        return result;
      }
      if (num != null) {
        result.push(num);
        k--;
      }
    }
  }

  return result;
};

console.log(topKFrequent([1, 2], 2));
