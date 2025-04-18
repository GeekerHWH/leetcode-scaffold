// two sum
function twoSum(array, target) {
    const hashMap = new Map()
    for (index in array) {
        let num = array[index]
        if (hashMap.has(target - num)){
            return [hashMap.get(target - num), index]
        }else{
            hashMap.set(num, index)
        }
    }
    return []
}

const testCase = [3,3]
console.log(twoSum(testCase, 6))
