
fn main() {
    let haystack = "mississippi";
    let needle = "issipi";

    println!("{:?}", s(haystack, needle));
}


fn s(haystack: &str, needle: &str) -> i32 {
    if haystack.len() < needle.len() {return -1}
    let haystack: Vec<char> = haystack.chars().collect();
    for (i, &c) in haystack.iter().enumerate() {
        if c == needle.chars().next().unwrap() {
            let mut matched = 1;
            for n in needle.chars().skip(1) {
                if i + matched >= haystack.len() { return -1 }
                if haystack[i + matched] == n {
                    matched += 1
                } else { break }
            }
            if matched == needle.len() {
                return i as i32;
            }
        }
    }

    -1
}