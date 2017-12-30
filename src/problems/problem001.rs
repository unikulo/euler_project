pub fn sum_of_multiples(x: i32, y: i32, below: i32) -> i32 {
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
    assert_eq!(sum_of_multiples(3, 5, 10), 23);
}