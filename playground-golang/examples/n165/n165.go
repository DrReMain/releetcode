package n165

func compareVersion(version1 string, version2 string) int {
	l1, l2 := len(version1), len(version2)
	x, y := 0, 0
	for x < l1 || y < l2 {
		n1, n2 := 0, 0
		for ; x < l1 && version1[x] != '.'; x++ {
			n1 = n1*10 + int(version1[x]-'0')
		}
		for ; y < l2 && version2[y] != '.'; y++ {
			n2 = n2*10 + int(version2[y]-'0')
		}
		if n1 > n2 {
			return 1
		}
		if n1 < n2 {
			return -1
		}

		x++
		y++
	}

	return 0
}
