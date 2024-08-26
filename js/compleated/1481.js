let sol = (sal) => { 
    sal.splice(sal.indexOf(Math.max(...sal)), 1)
    sal.splice(sal.indexOf(Math.min(...sal)), 1)
    
    let sum = 0
    for (let i of sal) {
        sum += i
    }
    return sum / sal.length
}

let sal = [4000,3000,1000,2000]


console.log(sol(sal))