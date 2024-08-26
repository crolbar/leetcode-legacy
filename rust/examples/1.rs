fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;

    println!("{:?}", s(nums, target));
}


fn s(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, &c) in nums.iter().enumerate() {
        for (i2, &c2) in nums.iter().enumerate() {
            if i != i2 {
                if c + c2 == target {
                    return vec![i as i32, i2 as i32];
                }
            }
        }
    }

    Vec::new()
}