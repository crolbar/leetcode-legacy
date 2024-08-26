fn main() {
    let n = "27346209830709182346";

    println!("{:?}", sol(n.to_string()));
}


fn sol(n: String) -> i32 {
    n.chars()
        .map(|n|
             n.to_digit(10).unwrap()
        )
    .max().unwrap() as i32
}
