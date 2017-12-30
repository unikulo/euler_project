pub fn largest_prime_factor(mut number: i64) -> i64 {
    let mut largest_prime_factor = 1;
    let mut factor = 2;
    while number > 1 {
        if number % factor == 0 {
            largest_prime_factor = factor;
            number /= factor;
        } else {
            factor += 1;
        }
    }
    largest_prime_factor
}
#[test]
fn test_largest_prime_factor(){
    assert_eq!(largest_prime_factor(13195), 29);
}