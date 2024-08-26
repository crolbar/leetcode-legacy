fn main() {
    //let (sentence1, sentence2) = ("qbaVXO Msgr aEWD v ekcb", "Msgr aEWD ekcb");
    let (sentence1, sentence2) = ("eTUny i b R UFKQJ EZx JBJ Q xXz", "eTUny i R EZx JBJ xXz");
    //let (sentence1, sentence2) = ("hsYZKp Cn eE", "hsYZKp eE");
    //let (sentence1, sentence2) = ("a A", "A A a a");

    println!("{:?}", sol(sentence1.to_string(), sentence2.to_string()));
}


fn sol(sentence1: String, sentence2: String) -> bool {
    if let 
        (Some(bigger), Some(smaller)) = 
            (
                [&sentence2, &sentence1].iter().max_by_key(|s| s.len()),
                [&sentence2, &sentence1].iter().min_by_key(|s| s.len())
            )
    {
        let s1 = bigger.split_whitespace().collect::<Vec<&str>>();
        let mut s2 = smaller.split_whitespace().collect::<Vec<&str>>();

        for (f, s) in s2.clone()
            .iter()
            .zip(s1.iter().clone())
        {
            if f == s {
                s2.remove(s2.iter().position(|i| i == s).unwrap());
            } else { break }
        }

        for (f, s) in s2.clone()
            .iter()
            .rev()
            .zip(s1.clone().iter().rev())
        {
            if f == s {
                s2.remove(s2.iter().position(|i| i == s).unwrap());
            } else { break }
        }

        println!("{:?}", s2);

        if s2.is_empty() {
            return true;
        }
    }
    false
}
