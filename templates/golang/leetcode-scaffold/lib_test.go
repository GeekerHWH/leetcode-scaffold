package leetcodescaffold_test

import (
	"testing"

	leetcodescaffold "github.com/GeekerHWH/leetcode-scaffold"
)

func TestTwoSum(t *testing.T) {
	result := leetcodescaffold.TwoSum([]int{2, 7, 11, 15}, 9)
	if result[0] != 0 || result[1] != 1 {
		t.Errorf("TwoSum([]int{2, 7, 11, 15}, 9) = %d; want [0,1]", result)
	}
}
