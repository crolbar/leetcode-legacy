fn main() {
    let nums = vec![2,4,1,1,6,5];


    println!("{:?}", sol(nums));
}


fn sol(nums: Vec<i32>) -> i32 {
    let mut num_hv = 0;

    let mut prev_hv: (usize, usize) = (0, 0);

    for (i, _) in nums.iter().enumerate().skip(1) {
        if i != nums.len() - 1 {
            let (mut left_i, mut right_i) = (i - 1, i + 1);

            while nums[left_i] == nums[i] {
                if left_i > 0 {
                    left_i -= 1;
                } else {
                    break;
                }

            }

            while nums[right_i] == nums[i] {
                if right_i < nums.len() - 1 {
                    right_i += 1;
                } else {
                    break;
                }
            }

            if 
                (nums[left_i] > nums[i] && nums[right_i] > nums[i]) ||
                (nums[left_i] < nums[i] && nums[right_i] < nums[i])
            {
                if prev_hv != (left_i, right_i) {
                    println!("left_i: {}, curr: {}, right_i: {}", nums[left_i], nums[i], nums[right_i]);
                    prev_hv = (left_i, right_i);
                    num_hv += 1;
                }
            }
        }
    }

    num_hv 
}
