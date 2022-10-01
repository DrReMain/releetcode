package leetcode

import "math"

func lengthOfLongestSubstring(s string) (l int) {
	m := map[byte]bool{}
	p := -1
	l = 0

	for i := 0; i < len(s); i++ {
		if i != 0 {
			delete(m, s[i-1])
		}

		for p+1 < len(s) && !m[s[p+1]] {
			m[s[p+1]] = true
			p++
		}

		l = int(math.Max(float64(l), float64(p-i+1)))
	}

	return
}
