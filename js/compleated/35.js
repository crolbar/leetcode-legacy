var s = function(nums, target) {
    nums.push(target)
    nums.sort(function(a, b) {
        return a - b
    })
    console.log(nums)
    for (let i = 0; i < nums.length; i++) {
        if (nums[i] == target) {
            return i
        }
    }
};

let nums = [1,2,3,4,5,10]
let target = 2

console.log(s(nums, target));
