/**
 * @param {number[]} nums
 * @return {number[]}
 */
var productExceptSelf = function (nums) {
  if (nums.length == 2) {
    return [nums[1], nums[0]];
  }

  let productWithoutZeros = 1;
  let zeros = 0;

  for (let i = 0; i < nums.length; i++) {
    if (nums[i] != 0) {
      productWithoutZeros *= nums[i];
    } else {
      zeros++;
    }
  }

  return nums.map((num) => {
    if (num == 0) {
      return zeros > 1 ? 0 : productWithoutZeros;
    } else {
      return zeros > 0 ? 0 : productWithoutZeros / num;
    }
  });
};
