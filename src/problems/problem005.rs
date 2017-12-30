// I know it's faster when using prime numbers but I'm to lazy to implement that for now.
pub fn smallest_multiple(to: i32) -> i32 {
    let mut result = to; // It can never be smaller
    'outer: loop {
        for i in 1..(to + 1) {
            let j = result % i;
            if j != 0 {
                result += i - j;
                continue 'outer;
            }
        }
        return result;
    }
}
#[test]
fn test_smallest_multiple() {
    assert_eq!(smallest_multiple(10), 2520);
}