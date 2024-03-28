class Solution {
  public int majorityElement(int[] nums) {
    int majorityElement = nums[0];
    int count = 1;

    for (int i = 1; i < nums.length; i++) {
      if (count == 0) {
        majorityElement = nums[i];
        count++;
      } else if (majorityElement == nums[i]) {
        count++;
      } else if (majorityElement != nums[i]) {
        count--;
      }
    }

    return majorityElement;
  }
}

class Program {
  public static void main(String[] args) {
    Solution solution = new Solution();
    int[] nums = { 10, 9, 9, 9, 10 };
    System.out.println(solution.majorityElement(nums));
  }
}