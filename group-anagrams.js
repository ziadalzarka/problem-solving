/**
 * @param {string[]} strs
 * @return {string[][]}
 */
var groupAnagrams = function (strs) {
  const map = {};
  for (let j = 0; j < strs.length; j++) {
    const str = strs[j];

    const count = new Array(26);
    for (let i = 0; i < str.length; i++) {
      const index = str.charCodeAt(i) - 97;
      count[index] = (count[index] || 0) + 1;
    }

    if (map[count]) {
      map[count].push(str);
    } else {
      map[count] = [str];
    }
  }

  return Object.values(map);
};
