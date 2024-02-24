package n490

import (
	"fmt"
)

func hasPath(maze [][]int, start []int, destination []int) bool {
	stack := [][]int{start}
	dirs := [][]int{{-1, 0}, {1, 0}, {0, 1}, {0, -1}}
	set := map[string]bool{fmt.Sprintf("%d#%d", start[0], start[1]): true}

	for len(stack) > 0 {
		x, y := stack[len(stack)-1][0], stack[len(stack)-1][1]
		stack = stack[:len(stack)-1]

		if x == destination[0] && y == destination[1] {
			return true
		}

		for _, dir := range dirs {
			i, j := x+dir[0], y+dir[1]
			for i >= 0 && i < len(maze) && j >= 0 && j < len(maze[0]) && maze[i][j] == 0 {
				i += dir[0]
				j += dir[1]
			}
			i -= dir[0]
			j -= dir[1]
			key := fmt.Sprintf("%d#%d", i, j)
			if !set[key] {
				stack = append(stack, []int{i, j})
				set[key] = true
			}
		}
	}

	return false
}
