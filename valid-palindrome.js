/**
 * @param {string} s
 * @return {boolean}
 */
var isPalindrome = function (s) {
  const alphanumeric =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
  const alphanumericSet = new Set(alphanumeric.split(""));

  let startPointer = 0;
  let endPointer = s.length - 1;

  while (startPointer < endPointer) {
    while (!alphanumericSet.has(s[startPointer]) && startPointer < endPointer) {
      startPointer++;
    }

    while (!alphanumericSet.has(s[endPointer]) && startPointer < endPointer) {
      endPointer--;
    }

    if (s[startPointer].toLowerCase() == s[endPointer].toLowerCase()) {
      startPointer++;
      endPointer--;
    } else {
      return false;
    }
  }

  return true;
};

console.log(isPalindrome(",."));
