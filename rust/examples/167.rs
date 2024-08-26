use std::collections::HashMap;

fn main() {
    let (nums, t) = (vec![1,2,3,4,4,9,56,90], 8);


    println!("{:?}", sol(nums, t))
}


fn sol(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, n) in numbers.iter().enumerate() {
        let c = target - n;

        if let Some(&x) = map.get(&c) {
            return vec![(x+1) as i32, (i+1) as i32];
        }

        map.insert(n, i);
    }

    vec![]
}
