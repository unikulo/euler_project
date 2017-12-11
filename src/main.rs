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
fn main() {
    println!("1: sum of all the multiples of 3 or 5 below 1000 = {0}", sum_of_multiples(3, 5, 1000));
    println!("2: sum of all the even numbers in the fibonacci sequence whose values do not exceed 4 million = {0}", sum_of_multiples_of_fibonacci_sequence(2, 4000000));
}
