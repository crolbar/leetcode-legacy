let sol = (s) => { 
    let length = 0
    for (let i = 1; i <= s.length; i++) {
        if (s[s.length - i] == " " && length != 0) {
            return length
        } else if (s[s.length - i] != " "){
            length++
        }
    }

    return length
}

let s = "s"
console.log(sol(s))