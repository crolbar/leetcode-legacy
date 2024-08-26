let sol = (x, y, bound) => { 
    let maxi = x == 1 ? 0 : Math.log(bound) / Math.log(x)
    let maxi2 = y == 1 ? 0 : Math.log(bound) / Math.log(y)
    let arr = []


    for (let i = 0; i <= maxi; i++) {
        for (let i2 = 0; i2 <= maxi2; i2++) {
            let pow = x ** i + y ** i2
            if (pow <= bound && !arr.includes(pow)) {
                arr.push(pow)
            } else if (pow > bound) {
                break
            }
        }
    }

    arr.sort((x, y) => {return x - y})


    return arr
}

let x = 2, y = 3, bound = 10


console.log(sol(x, y, bound))


