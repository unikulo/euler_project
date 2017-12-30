pub fn largest_palindrome_product(n: u32) -> i32{
    let mut largest_palindrome_product: i32 = 0;
    for i in 10_i32.pow(n-1)..10_i32.pow(n) {
        for j in 10_i32.pow(n-1)..10_i32.pow(n) {
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