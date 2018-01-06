pub fn nth_prime_number(n: i32) -> i32{
    let mut prime_numbers: Vec<i32> = Vec::new();
    let mut prime = 2;
    let mut count = 1;
    'prime_test: loop {
        for i in prime_numbers.iter() {
            if prime % i == 0 {
                prime += 1;
                continue 'prime_test;
            }
        }
        if count == n {
            break;
        } else {
            prime_numbers.push(prime);
            count += 1;
        }
    }
    prime
}
#[test]
fn test_nth_prime_number() {
    assert_eq!(nth_prime_number(6), 13)
}