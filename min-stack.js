var MinStack = function () {
  this.stack = [];
  this.minimumStack = [];
};

/**
 * @param {number} val
 * @return {void}
 */
MinStack.prototype.push = function (val) {
  this.stack.push(val);
  const currentMinimum = this.getMin();
  if (val < currentMinimum || currentMinimum === undefined) {
    this.minimumStack.push(val);
  } else {
    this.minimumStack.push(currentMinimum);
  }
};

/**
 * @return {void}
 */
MinStack.prototype.pop = function () {
  this.minimumStack.pop();
  return this.stack.pop();
};

/**
 * @return {number}
 */
MinStack.prototype.top = function () {
  return this.stack[this.stack.length - 1];
};

/**
 * @return {number}
 */
MinStack.prototype.getMin = function () {
  return this.minimumStack[this.minimumStack.length - 1];
};
