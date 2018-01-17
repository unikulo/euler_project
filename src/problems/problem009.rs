pub fn find_pythagorean_triplet(sum: i32) -> i32{
    let mut a = 1;
    let mut b = sum - 1;
    loop {
        let c = sum - b - a;
        let c_squared = c*c;
        let c2 = a*a + b*b;
        if c2 == c_squared {
            return a * b * c;
        }else if c2 > c_squared {
            b -= 1;
        } else {
            a += 1;
        }
    }
}