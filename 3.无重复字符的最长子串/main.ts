function lengthOfLongestSubstring(s: string): number {
  const m = new Map();
  let p = -1;
  let len = 0; // 字串长度

  // 从第一个字符开始遍历，寻找子串
  // i为子串第一个字符下标
  for (let i = 0; i < s.length; i++) {
    // 找到第一个子串之后
    // 下一个子串需要删除map中，的本次子串可能存在的字符，避免影响本次字串的寻找
    if (i !== 0) m.delete(s[i - 1]);

    // 寻找子串，需要p作为临时指针，指向当前字符
    // 对当前字符判断和处理
    // 如果map中有当前字符，代表当前子串开始有重复字符
    // map中没有当前字符, 就往map中设置，并继续遍历下一个字符，直到寻找到本次子串
    while (p + 1 < s.length && !m.has(s[p + 1])) {
      m.set(s[p + 1], true);
      p++;
    }

    // 当前子串判断结束，取上次和本次子串长的值
    // p是当前子串最后一个字符的下标
    // p-i + 1 即为长度
    len = Math.max(len, p - i + 1);
  }

  return len;
}
