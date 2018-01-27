pub fn nth_prime_number(n: usize) -> usize{
    let mut size: usize = 100;
    loop {
        let mut vec = Vec::with_capacity(size - 2);
        for _ in 2..size {
            vec.push(true);
        }
        let mut i = 2;
        let root: f64 = (size as f64).sqrt();
        loop {
            if i == size {
                break;
            }
            if vec[i - 2] == false {
                i += 1;
                continue;
            }
            if i as f64 > root {
                break;
            }
            let mut j = i * i;
            loop {
                vec[j - 2] = false;
                j += i;
                if j >= size {
                    break;
                }
            }
            i += 1;
        }
        let mut number_of_primes = 0;
        for (i, b) in vec.iter().enumerate() {
            if b == &true {
                number_of_primes += 1;
                if number_of_primes == n {
                    return i + 2;
                }
            }
        }
        size *= 10;
    }
}
#[test]
fn test_nth_prime_number() {
    assert_eq!(nth_prime_number(6), 13)
}