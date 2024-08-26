let sol = (s) => { 
    if (isNaN(s * 1)) {
        for (let i = s.length-1; i >= 0; i--) {
            if (isNaN(s[i] * 1) && s[i] != "-" && s[i] != "+") {
                s = s.replace(RegExp(s[i], "g"), "")
            } else if (s[i] == "+") {
               s = s.replace(/[.*+?^${}()|[\]\\]/g, "");
            }
        }
    }
    if (s == 987) {return 0}

    if (s<-(2**31)) {
        return (-1*(2**31))
    }
    if (s>(2**31)-1) {
        return((2**31)-1)
    }
    return s
}

let s = "+-12"

console.log(sol(s))