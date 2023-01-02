package leetcode

func finalValueAfterOperations(operations []string) int {
	result := 0

	for _, ope := range operations {
		ans := int(',') - int(ope[1])
		result += ans
	}

	return result
}
