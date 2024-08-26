
fn main() {
    let nums = vec![1,3,5,6];
    let target = 1;

    println!("{:?}", s(nums, target));
}


fn s(nums: Vec<i32>, target: i32) -> i32 {
    let mut n = nums.clone();
    if !n.contains(&target) {
        n.push(target);
        n.sort();
    }
    n.binary_search(&target).unwrap() as i32
}