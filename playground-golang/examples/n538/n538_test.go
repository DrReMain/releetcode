package n538

import (
	"reflect"
	"testing"

	"drremain.cn/playground-golang/DataStructure"
)

func TestConvertBST1(t *testing.T) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{4, 1, 6, 0, 2, 5, 7, nil, nil, nil, 3, nil, nil, nil, 8})
	expected := DataStructure.NewTreeNodeFromSlice([]interface{}{30, 36, 21, 36, 35, 26, 15, nil, nil, nil, 33, nil, nil, nil, 8})
	if got := convertBST(root); !reflect.DeepEqual(got, expected) {
		t.Errorf("Expected %v, but got %v for input %#v", expected, got, root)
	}
}

func TestConvertBST2(t *testing.T) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{0, nil, 1})
	expected := DataStructure.NewTreeNodeFromSlice([]interface{}{1, nil, 1})
	if got := convertBST(root); !reflect.DeepEqual(got, expected) {
		t.Errorf("Expected %v, but got %v for input %#v", expected, got, root)
	}
}

func TestConvertBST3(t *testing.T) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{1, 0, 2})
	expected := DataStructure.NewTreeNodeFromSlice([]interface{}{3, 3, 2})
	if got := convertBST(root); !reflect.DeepEqual(got, expected) {
		t.Errorf("Expected %v, but got %v for input %#v", expected, got, root)
	}
}

func TestConvertBST4(t *testing.T) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{3, 2, 4, 1})
	expected := DataStructure.NewTreeNodeFromSlice([]interface{}{7, 9, 4, 10})
	if got := convertBST(root); !reflect.DeepEqual(got, expected) {
		t.Errorf("Expected %v, but got %v for input %#v", expected, got, root)
	}
}

func BenchmarkConvertBST(b *testing.B) {
	root := DataStructure.NewTreeNodeFromSlice([]interface{}{4, 1, 6, 0, 2, 5, 7, nil, nil, nil, 3, nil, nil, nil, 8})
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		convertBST(root)
	}
}
