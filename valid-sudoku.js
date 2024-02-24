/**
 * @param {character[][]} board
 * @return {boolean}
 */
var isValidSudoku = function (board) {
  const squaresHashmaps = [
    [{}, {}, {}],
    [{}, {}, {}],
    [{}, {}, {}],
  ];
  const columnsHashmaps = [{}, {}, {}, {}, {}, {}, {}, {}, {}];
  let currentRowHashmap = {};

  for (let rowIndex = 0; rowIndex < 9; rowIndex++) {
    currentRowHashmap = {};

    for (let columnIndex = 0; columnIndex < 9; columnIndex++) {
      const currentCell = board[rowIndex][columnIndex];

      if (currentCell == ".") {
        continue;
      }

      if (currentRowHashmap[currentCell]) {
        return false;
      }

      currentRowHashmap[currentCell] = true;

      const squareHashmap =
        squaresHashmaps[Math.floor(rowIndex / 3)][Math.floor(columnIndex / 3)];

      if (squareHashmap[currentCell]) {
        return false;
      }

      squareHashmap[currentCell] = true;

      const columnHashmap = columnsHashmaps[columnIndex];

      if (columnHashmap[currentCell]) {
        return false;
      }

      columnHashmap[currentCell] = true;
    }
  }

  return true;
};

console.log(
  isValidSudoku([
    ["5", "3", ".", ".", "7", ".", ".", ".", "."],
    ["6", ".", ".", "1", "9", "5", ".", ".", "."],
    [".", "9", "8", ".", ".", ".", ".", "6", "."],
    ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
    ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
    ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
    [".", "6", ".", ".", ".", ".", "2", "8", "."],
    [".", ".", ".", "4", "1", "9", ".", ".", "5"],
    [".", ".", ".", ".", "8", ".", ".", "7", "9"],
  ]),
);
