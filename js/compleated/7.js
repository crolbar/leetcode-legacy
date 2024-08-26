let sol = (s) => { 
    s = s.toString()
    let str = new String
    if (s[0] == '-') {str += '-'}

    for (let i = 0; i < s.length; i++) {
        if (s[s.length - 1 - i] != "-") {
            str += s[s.length - 1 - i] 
        }
    }

    if (str >= 2147483648 || str <= -2147483648) {
        return 0
    }

    return str
}

let s = -129334

console.log(sol(s))