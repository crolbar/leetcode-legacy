fn main() {
    let x = 122;

    println!("{}", s(x));
}


fn s(x: i32) -> bool {
    let mut y = String::new();
    for i in x.to_string().chars().rev() {
        y += i.to_string().as_str();
    }

    x == y.parse::<i32>().unwrap_or_else(|_| 1)
}