function finalValueAfterOperations(operations: string[]): number {
  let result = 0;

  operations.forEach((ope) => {
    result += ",".charCodeAt(0) - ope.charCodeAt(1);
  });

  return result;
}
