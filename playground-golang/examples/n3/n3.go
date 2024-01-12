package n3

import (
	"drremain.cn/playground-golang/pkg/compare"
)

var max = compare.MaxInt

func lengthOfLongestSubstring(s string) (ret int) {
	length := len(s)
	if length < 2 {
		return length
	}

	m := make(map[byte]int, length)
	for p, i := 0, 0; i < length; i++ {
		if idx, ok := m[s[i]]; ok {
			p = max(p, idx+1)
		}
		ret = max(ret, i-p+1)
		m[s[i]] = i
	}

	return
}
