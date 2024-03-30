/**
 * @param {string[]} tokens
 * @return {number}
 */
var evalRPN = function (tokens) {
  const stack = [];

  let leftSideOperand = null;
  let rightSideOperand = null;

  for (const token of tokens) {
    if (!isNaN(token)) {
      stack.push(Number.parseInt(token, 10));
    } else {
      rightSideOperand = stack.pop();
      leftSideOperand = stack.pop();

      switch (token) {
        case "+":
          stack.push(leftSideOperand + rightSideOperand);
          break;
        case "-":
          stack.push(leftSideOperand - rightSideOperand);
          break;
        case "*":
          stack.push(leftSideOperand * rightSideOperand);
          break;
        case "/":
          const divisionOutput = leftSideOperand / rightSideOperand;
          if (divisionOutput > 0) {
            stack.push(Math.floor(divisionOutput));
          } else {
            stack.push(Math.ceil(divisionOutput));
          }
          break;
      }
    }
  }

  return stack.pop();
};

console.log(
  evalRPN([
    "10",
    "6",
    "9",
    "3",
    "+",
    "-11",
    "*",
    "/",
    "*",
    "17",
    "+",
    "5",
    "+",
  ]),
);
