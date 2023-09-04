package leetcode

import (
	"strconv"
	"strings"
	"math"
)

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

type Codec struct{}

func Constructor() Codec {
	return Codec{}
}

func (this *Codec) serialize(root *TreeNode) string {
	list := []string{}

	var iter func(*TreeNode, *[]string)
	iter = func(_node *TreeNode, _list *[]string) {
		if _node == nil {
			return
		}

		iter(_node.Left, _list)
		iter(_node.Right, _list)
		*_list = append(*_list, strconv.Itoa(_node.Val))
	}

	iter(root, &list)

	return strings.Join(list, "-")
}

func (this *Codec) deserialize(data string) *TreeNode {
    if data == "" {
        return nil
    }

    list := strings.Split(data, "-")

    var construct func(int, int) *TreeNode
    construct = func(lower, upper int) *TreeNode {
        if len(list) == 0 {
            return nil
        }

        val, _ := strconv.Atoi(list[len(list)-1])

        if val < lower || val > upper {
            return nil
        }

        list = list[:len(list)-1]
		
        return &TreeNode{Val: val, Right: construct(val, upper), Left: construct(lower, val)}
    }

    return construct(math.MinInt32, math.MaxInt32)
}
