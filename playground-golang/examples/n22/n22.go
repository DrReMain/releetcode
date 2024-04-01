package n22

func generateParenthesis(n int) (ret []string) {
	var backtrack func(string, int, int)
	backtrack = func(path string, left, right int) {
		if len(path) == n*2 {
			ret = append(ret, path)
			return
		}

		if left < n {
			backtrack(path+"(", left+1, right)
		}
		if right < left {
			backtrack(path+")", left, right+1)
		}
	}
	backtrack("", 0, 0)
	return
}
