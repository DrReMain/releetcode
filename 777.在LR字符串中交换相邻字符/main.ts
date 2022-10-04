function canTransform(start: string, end: string): boolean {
  const sLen = start.length;
  let i = 0;
  let j = 0;

  while (i < sLen && j < sLen) {
    // 遍历start和end非"X"的字符
    while (i < sLen && start[i] === "X") {
      i++;
    }
    while (j < sLen && end[j] === "X") {
      j++;
    }

    // 如果不等，一定返回false
    if (start[i] !== end[j]) {
      return false;
    }

    // 如果相等，判断当前字符
    // 为L，且在start中较早出现，则一定返回false
    // 为R，且在end中较早出现，则一定返回false
    const c = start[i];
    if ((c === "L" && i < j) || (c === "R" && i > j)) {
      return false;
    }
    i++;
    j++;
  }

  // 当end遍历完，start中还有非X字符一定返回false
  while (i < sLen) {
    if (start[i] !== "X") {
      return false;
    }
    i++;
  }

  // 当start遍历完，end中还有非X字符一定返回false
  while (j < sLen) {
    if (end[j] !== "X") {
      return false;
    }
    j++;
  }
  return true;
}
