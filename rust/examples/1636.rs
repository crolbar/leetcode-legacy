fn main() {
    let nums = vec![-1,1,-6,4,5,-6,1,4,1];

    println!("{:?}", sol(nums));
}


fn sol(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums; nums.sort();
    let mut map: Vec<(i32, i32)> = vec![];

    let mut curr_temp = nums[0];
    let mut curr_count = 0;

    for i in nums {
        if i != curr_temp {
            map.push((curr_temp, curr_count));

            curr_temp = i;
            curr_count = 1;
        } else {
            curr_count += 1;
        }
    }
    map.push((curr_temp, curr_count));
    map.sort_by_key(|&(_, v)| v);
    map.sort_by(|&(k, v), &(k2, v2)| 
        if v == v2 { 
            k2.cmp(&k)
        } else {
            v.cmp(&v2)
        }
    );

    println!("{:?}", map);

    map.iter().flat_map(|(k, v)| 
        std::iter::repeat(*k).take(*v as usize).collect::<Vec<i32>>()
    ).collect()
}
