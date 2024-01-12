package main

import (
	"fmt"

	"drremain.cn/playground-golang/pkg/compare"
)

func main() {
	a, b := 1, 2
	fmt.Println(compare.MaxInt(a, b))
	fmt.Println(compare.MinInt(a, b))
}
