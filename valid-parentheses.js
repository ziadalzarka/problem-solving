/**
 * @param {string} s
 * @return {boolean}
 */
var isValid = function (s) {
  const stack = [];

  for (const char of s) {
    if (char === "(" || char === "[" || char === "{") {
      stack.push(char);
    } else if (char === ")") {
      if (stack.pop() !== "(") {
        return false;
      }
    } else if (char === "}") {
      if (stack.pop() !== "{") {
        return false;
      }
    } else if (char === "]") {
      if (stack.pop() !== "[") {
        return false;
      }
    }
  }

  if (stack.length !== 0) {
    return false;
  }

  return true;
};

console.log(isValid("()"));
console.log(isValid("()[]{}"));
console.log(isValid("([])"));
