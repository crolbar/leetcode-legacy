fn main() {
    let s = "{}{{()}{})".to_string();

    println!("{:?}", sol(s))
}


fn sol(s: String) -> bool {
    let mut vec = Vec::new();
    for c in s.chars() {
        match c {
            '{' => { vec.push(c) }
            '[' => { vec.push(c)  }
            '(' => { vec.push(c) }
            '}' =>  { if Some('{') != vec.pop() { return false } },
            ']' => { if Some('[') != vec.pop() { return false } },
            ')' => { if Some('(') != vec.pop() { return false } },
            _ => {}
        }
    }

    vec.is_empty()
}