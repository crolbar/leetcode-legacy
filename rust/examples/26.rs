use std::borrow::BorrowMut;

fn main() {
    let mut nums = vec![0,0,1,1,1,2,2,3,3,4];


    println!("{:?}", s(nums.borrow_mut()));
}


fn s(nums: &mut Vec<i32>) -> i32 {
    for (i, _) in nums.clone().iter().enumerate().rev() {
        if i != 0 && nums[i] == nums[i - 1] {
            nums.remove(i);
        }
    }

    nums.len() as i32
}