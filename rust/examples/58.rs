
fn main() {
    let s = "   fly me   to   the moon  ".to_string();

    println!("{:?}", sol(s));
}


fn sol(s: String) -> i32 {
    let mut len = 0;

    for i in s.chars().rev() {
        if i == ' ' && len != 0 {
            return len
        } else if i != ' ' {
           len += 1 
        }
    }

    len
}
