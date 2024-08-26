fn main() {
    let nums = vec![1,2,3,2,3];

    println!("{:?}", sol(nums));
}


fn sol(nums: Vec<i32>) -> i32 {
    nums.iter().enumerate()
        .filter(|(i, n)| 
            !nums.iter().enumerate()
                .filter(|(i2, _)| i != i2)
                .find(|(_, j)| {
                    n == j 
                }).is_some()
        )
    .map(|(_, n)| n)
    .sum()
}
