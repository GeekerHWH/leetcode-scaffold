package leetcodescaffold

func TwoSum(nums []int, target int) []int {
	hashmap := make(map[int]int)
	for i, num := range nums {
		if prev, ok := hashmap[target-num]; ok {
			return []int{prev, i}
		}
		hashmap[num] = i
	}
	return []int{}
}
