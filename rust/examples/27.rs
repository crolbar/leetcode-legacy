use std::borrow::BorrowMut;

fn main() {
    let mut nums: Vec<i32> = vec![3,2,2,3];
    let val = 3;

    println!("{:?}", s(nums.borrow_mut(), val));
}


fn s(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|v| v != &val);
    nums.len() as i32
}