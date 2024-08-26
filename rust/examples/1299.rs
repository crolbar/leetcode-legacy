fn main() {
    let nums = vec![17,18,5,4,6,1];

    println!("{:?}", sol(nums));
}


fn sol(arr: Vec<i32>) -> Vec<i32> {
    arr.iter().enumerate().map(|(i, _)| {
        if i == arr.len() - 1 {return -1};
        let mut lm = 0;

        for n in (i + 1)..arr.len() {
            let ln = arr[n];

            if lm < ln {
                lm = ln;
            }
        }

        lm
    }).collect()
}
