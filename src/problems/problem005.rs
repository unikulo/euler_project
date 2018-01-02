// I'm sorry of this is a bit messy. I don't really understand borrowing and moving yet.
pub fn smallest_multiple(to: i32) -> i32 {
    let mut prime_numbers: Vec<i32> = Vec::new();
    for i in 2..(to + 1) {
        let mut i = i;
        let mut j = 2;
        let mut primes: Vec<i32> = Vec::new();
        loop {
            if i % j == 0 {
                primes.push(j);
                i = i / j;
                if i == 1 {
                    break;
                }
            } else {
                j += 1;
            }
        }
        for prime in primes.clone().iter() {
            let a = occurs(prime_numbers.clone(), &prime);
            let b = occurs(primes.clone(), &prime);
            let mut c = b - a;
            while c > 0 {
                prime_numbers.push(*prime);
                c -= 1;
            }
        }
    }
    let mut result: i32 = 1;
    for number in prime_numbers.iter(){
        result *= number;
    }
    result
}
fn occurs(v: Vec<i32>, number: &i32) -> i32{
    let mut count: i32 = 0;
    for x in v.iter(){
        if x == number {
            count += 1;
        }
    }
    count
}
#[test]
fn test_smallest_multiple() {
    assert_eq!(smallest_multiple(10), 2520);
}