pub fn nth_prime_number(n: usize) -> i32{
    let mut prime_numbers: Vec<i32> = Vec::with_capacity(n - 1);
    let mut prime = 3;
    let mut count = 2;
    'prime_test: loop {
        for i in prime_numbers.iter() {
            if prime % i == 0 {
                prime += 2;
                continue 'prime_test;
            }
        }
        if count == n {
            break;
        } else {
            prime_numbers.push(prime);
            count += 1;
            prime += 2;
        }
    }
    prime
}
#[test]
fn test_nth_prime_number() {
    assert_eq!(nth_prime_number(6), 13)
}