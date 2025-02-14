// Find the product abc of a Pythagorean triplet for which a + b + c = 1000.
// a^2 + b^2 = c^2
// a, b, c are integers and a < b < c

pub fn special_pythagorean_triplet() -> u64 {
    for a in 1..1000 {
        for b in a..1000 {
            if (a + b) < 1000 {
                let c = 1000 - a - b;
                if a * a + b * b == c * c {
                    return a * b * c;
                }
            }
        }
    }
    0
}
