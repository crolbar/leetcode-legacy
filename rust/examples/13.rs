 fn main() {
    let s = "MCMXCIV".to_string();


    
    println!("{}", sol(s));
 }
 
fn sol(s: String) -> i32 {
    let mut sum = 0;
    let mut last_num = 0;
    for c in s.chars() {
        match (last_num, c) {
            (1, 'V') => sum += 3,
            (1, 'X') => sum += 8,
            (10, 'L') => sum += 30,
            (10, 'C') => sum += 80,
            (100, 'D')=> sum += 300,
            (100, 'M') => sum += 800,
            _ => {
                match c {
                    'I' => {sum += 1; last_num = 1},
                    'V' => sum += 5,
                    'X' => {sum += 10; last_num = 10},
                    'L' => sum += 50,
                    'C' => {sum += 100; last_num = 100},
                    'D' => sum += 500,
                    'M' => sum += 1000,
                    _ => ()
                }
            }
        }
    }
    sum
}