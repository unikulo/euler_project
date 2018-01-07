mod problems;

use problems::problem001;
use problems::problem002;
use problems::problem003;
use problems::problem004;
use problems::problem005;
use problems::problem006;
use problems::problem007;
use problems::problem008;
use problems::problem009;

fn main() {
    println!("1: sum of all the multiples of 3 or 5 below 1000 = {0}", problem001::sum_of_multiples(3, 5, 1000));
    println!("2: sum of all the even numbers in the fibonacci sequence whose values do not exceed 4 million = {0}", problem002::sum_of_multiples_of_fibonacci_sequence(2, 4000000));
    println!("3: largest prime factor of the number 600851475143 = {0}", problem003::largest_prime_factor(600851475143));
    println!("4: largest palindrome made from the product of two 3-digit numbers = {0}", problem004::largest_palindrome_product(3));
    println!("5: smallest positive number that is evenly dividable by all of the numbers from 1 to 20 = {}", problem005::smallest_multiple(20));
    println!("6: difference between the sum of the squares of the first one hundred natural numbers and the square of the sum = {}", problem006::sum_square_difference(100));
    println!("7: the 10001th prime number = {}", problem007::nth_prime_number(10001));
    println!("8: the largest product of thirteen adjacent digits in the 1000-digit number = {}", problem008::largest_product_in_a_series(13));
    println!("9: The product of Pythagorean triplet for which a + b + c = 1000, = {}", problem009::find_pythagorean_triplet(1000));
}
