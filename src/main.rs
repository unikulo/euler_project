mod problems;

use problems::problem001;
use problems::problem002;
use problems::problem003;
use problems::problem004;
use problems::problem005;

fn main() {
    println!("1: sum of all the multiples of 3 or 5 below 1000 = {0}", problem001::sum_of_multiples(3, 5, 1000));
    println!("2: sum of all the even numbers in the fibonacci sequence whose values do not exceed 4 million = {0}", problem002::sum_of_multiples_of_fibonacci_sequence(2, 4000000));
    println!("3: largest prime factor of the number 600851475143 = {0}", problem003::largest_prime_factor(600851475143));
    println!("4: largest palindrome made from the product of two 3-digit numbers = {0}", problem004::largest_palindrome_product(1000));
    println!("5: smallest positive number that is evenly dividable by all of the numbers from 1 to 20 = {}", problem005::smallest_multiple(20));
}
