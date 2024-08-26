let h = 13

if (h >= 12) {
    h = (h == 12 || h == 24) ? 12 : h%12
}

console.log(h);