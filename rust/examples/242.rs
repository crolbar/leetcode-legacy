fn main() {
    let (s, t) = ("aacc", "ccac");


    println!("{:?}", sol(s.to_string(), t.to_string()));
}


fn sol(s: String, t: String) -> bool {
    if s.len() != t.len() {return false}
    let mut t = t;
    for i in s.bytes() {
        if let Some(match_index) = t.as_bytes().iter().position(|b| b == &i) {
           t.remove(match_index);
        } else {
            return false;
        }
    }
    true
}
