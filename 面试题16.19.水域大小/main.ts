function pondSizes(land: number[][]): number[] {
    const row = land.length;
    const col = land[0].length;
  
    const list: number[] = [];
  
    for (let i = 0; i < row; i++) {
      for (let j = 0; j < col; j++) {
        if (land[i][j] === 0) {
          // list.push(dfs(land, i, j));
          list.push(bfs(land, i, j));
        }
      }
    }
    
    return list.sort((a, b) => a - b);
  };
  
  interface ISearch {
    (_: number[][], __: number, ___: number): number
  }
  
  const dfs: ISearch = (land, x, y) => {
    const row = land.length;
    const col = land[0].length;
  
    if (x < 0 || x >= row || y < 0 || y >= col || land[x][y] !== 0) {
      return 0;
    }
  
    land[x][y] = -1;
  
    let result = 1;
    for (let i = -1; i <= 1; i++) {
      for (let j = -1; j <= 1; j++) {
        if (i === 0 && j === 0) {
          continue;
        }
        result += dfs(land, x + i, y + j);
      }
    }
  
    return result;
  }
  
  const bfs: ISearch = (land, x, y) => {
    const row = land.length;
    const col = land[0].length;
  
    let result = 0;
    const queue: [number, number][] = [];
  
    queue.push([x, y]);
    land[x][y] = -1;
  
    while (queue.length > 0) {
      const arr = queue.shift();
      const curX = arr[0];
      const curY = arr[1];
      result++;
  
      for (let i = -1; i <= 1; i++) {
        for (let j = -1; j <= 1; j++) {
          if (i === 0 && j === 0) {
            continue;
          }
  
          if ( curX + i < 0 || curX + i >= row || curY + j < 0 || curY + j >= col || land[curX+i][curY+j] !== 0 ) {
            continue;
          }
  
          land[curX+i][curY+j] = -1;
          queue.push([curX+i, curY+j]);
        }
      }
    }
  
    return result;
  }