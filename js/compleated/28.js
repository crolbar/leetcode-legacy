
var s = function(haystack, needle) {
    for (let h = 0; h < haystack.length; h++) {
        if (haystack.charAt(h) == needle.charAt(0)) {
            let matched = 1
            for (let n = 1; n < needle.length; n++) {
                if (haystack.charAt(h + n) == needle.charAt(n)) {
                    matched++
                } else {
                    continue
                }
            }
            if (matched == needle.length) {
                return h
            }
        }
    }
    
    return -1
};

let haystack = "mississippi"
let needle = "issip"

console.log(s(haystack, needle));
