/*
 * @lc app=leetcode.cn id=1410 lang=golang
 *
 * [1410] HTML 实体解析器
 */

// @lc code=start
func entityParser(text string) string {
	m := map[string]string{
		"quot":  "\"",
		"apos":  "'",
		"amp":   "&",
		"gt":    ">",
		"lt":    "<",
		"frasl": "/",
	}

	re := regexp.MustCompile(`&(quot|apos|amp|gt|lt|frasl);`)

	return re.ReplaceAllStringFunc(text, func(match string) string {
		group := strings.TrimSuffix(strings.TrimPrefix(match, "&"), ";")
		if replacement, ok := m[group]; ok {
			return replacement
		}
		return match
	})
}

// @lc code=end

