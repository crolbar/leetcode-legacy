let sol = (s) => { 
    let longest = [0]
    let arr = []

    for (let i = 0; i < s.length; i++) {
        arr.push(s[i])
        for (let c = i + 1; c < s.length; c++) {
            if (arr.includes(s[c])) {
                longest.push(arr.length)
                arr = []
                break
            }

            if (!arr.includes(s[c])) {
                arr.push(s[c])
            }
        }
        longest.push(arr.length)
        arr = []
    }


    return Math.max(...longest)
}

let s = "pwwekew"


console.log(sol(s))