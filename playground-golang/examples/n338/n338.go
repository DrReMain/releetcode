package n338

func countBits(n int) []int {
	ret := make([]int, n+1)
	for i := 0; i <= n; i++ {
		count, remain := 0, i
		for remain > 0 {
			if remain % 2 != 0 {
				count++
			}
			remain >>= 1
		}
		ret[i] = count
	}
	return ret
}