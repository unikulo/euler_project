pub fn sum_of_multiples_of_fibonacci_sequence(multiple: i32, below: i32) -> i32{
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