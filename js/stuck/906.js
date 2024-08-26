let sol = (left, right) => { 
    left = parseInt(left)
    right = parseInt(right)


    let c = 0
    for (let i = left; i < right; i++) {
        if (isPalindrome(i) && isPalindrome(Math.sqrt(i))) {
            c++
        }
    }
    return c
}

let isPalindrome = (num) => {
    if (!Number.isInteger(num)) {
        return false
    }
    num = num.toString()


    for (let i in num) { 
        if (num[num.length - 1 - i] == num[i]) {
            // console.log(num[num.length - 1 - i], num[i]);
            return true
        }
    }

    return false
}

let left = "40000000000000000", right = "50000000000000000"


console.log(sol(left, right))


