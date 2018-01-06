pub fn largest_palindrome_product(n: u32) -> i32{
    let from: i32 = 10_i32.pow(n-1);
    let to: i32 = 10_i32.pow(n);
    let mut largest_palindrome_product: i32 = 0;
    for i in from..to {
        for j in from..to {
            if j > i {
                break;
            }
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
    assert_eq!(largest_palindrome_product(2), 9009);
}