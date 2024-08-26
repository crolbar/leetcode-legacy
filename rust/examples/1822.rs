
fn main() {
    let nums = vec![51,38,73,21,27,55,18,15,79,29,13,45,8,-73,-92,-20,-50,-60,-70];

    // println!("{:?}", sol(nums));
    println!("{}", 25 / -25)
}


fn sol(nums: Vec<i32>) -> i32 {
    if nums.contains(&0) {
        return 0;
    }
    let mut prod = 1;
    for i in nums {
        if i > 0 {
            prod *= 1 
        } else {
            prod *= -1 
        }
    }
    prod
}
