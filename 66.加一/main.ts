function plusOne(digits: number[]): number[] {
  let length = digits.length;
  let flag = 1;

  for (let i = length - 1; i >= 0; i--) {
    let ans = digits[i] + flag;

    if (ans >= 10) {
      flag = 1;
    } else {
      flag = 0;
    }

    digits[i] = ans % 10;
  }

  if (flag > 0) {
    return [1, ...digits];
  } else {
    return digits;
  }
}
