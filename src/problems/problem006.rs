pub fn sum_square_difference(to: i32) -> i32{
    ((1 + to) * to / 2).pow(2) - sum_vec((1..to+1).map(|x|x.pow(2)).into_iter().collect())
}
fn sum_vec(vec: Vec<i32>) -> i32{
    let mut sum: i32 = 0;
    for i in vec {
        sum+= i;
    }
    sum
}
#[test]
fn test_sum_square_difference(){
    assert_eq!(sum_square_difference(10), 2640)
}