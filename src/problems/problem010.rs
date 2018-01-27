pub fn sum_of_primes_below(n: usize) -> usize {
    let mut vec = Vec::with_capacity((n - 2) as usize);
    for _ in 2..n {
       vec.push(true);
    }
    let mut i = 2;
    let root: f64 = (n as f64).sqrt();
    loop {
        if i == n {
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
            if j >= n {
                break;
            }
        }
        i += 1;
    }
    let mut result: usize = 0;
    for (i, b) in vec.iter().enumerate() {
        if b == &true {
            result += i + 2;
        }
    }
    result
}
#[test]
fn test_sum_of_primes_below() {
    assert_eq!(sum_of_primes_below(10), 17)
}