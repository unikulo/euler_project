fn sum_of_multiples(x: i32, y: i32, below: i32) -> i32 {
    let mut sum: i32 = 0;
    for i in 1..below {
        if i % x == 0 || i % y == 0{
            sum+= i;
        }
    }
    sum
}
#[test]
fn test_sum_of_multiples() {
    assert_eq!(sum_of_multiples(3,5,10), 23);
}
fn sum_of_multiples_of_fibonacci_sequence(multiple: i32, below: i32) -> i32{
    let mut sum: i32 = 0;
    let mut a = 1;
    let mut b = 2;
    while a < below{
        if a % multiple == 0 {
            sum+= a;
        }
        let c = a + b;
        a = b;
        b = c;
    }
    sum
}
#[test]
fn test_sum_of_multiples_of_fibonacci_sequence(){
    assert_eq!(sum_of_multiples_of_fibonacci_sequence(2, 100), 44);
    assert_eq!(sum_of_multiples_of_fibonacci_sequence(3, 100), 24);
    assert_ne!(sum_of_multiples_of_fibonacci_sequence(3, 100), 44);
}
fn largest_prime_factor(mut number: i64) -> i64 {
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
fn largest_palindrome_product(max_product: i32) -> i32{
    let mut largest_palindrome_product: i32 = 0;
    for i in 1..max_product {
        for j in 1..max_product {
            let product = i * j;
            let string = product.to_string();
            if string.bytes().eq(string.bytes().rev()) {
                if product > largest_palindrome_product {
                    largest_palindrome_product = product;
                }
            }
        }
    }
    largest_palindrome_product
}
#[test]
fn test_largest_palindrome_product(){
    assert_eq!(largest_palindrome_product(100), 9009);
}
fn main() {
    println!("1: sum of all the multiples of 3 or 5 below 1000 = {0}", sum_of_multiples(3, 5, 1000));
    println!("2: sum of all the even numbers in the fibonacci sequence whose values do not exceed 4 million = {0}", sum_of_multiples_of_fibonacci_sequence(2, 4000000));
    println!("3: largest prime factor of the number 600851475143 = {0}", largest_prime_factor(600851475143));
    println!("4: largest palindrome made from the product of two 3-digit numbers = {0}", largest_palindrome_product(1000));
}
