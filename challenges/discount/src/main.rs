/* 
Find the Discount:
    Create a function that takes two arguments as integers: 
        * the original price,
        * the discount percentage,
    and returns:
        * the final price after the discount.
Examples:
    dis(1500, 50) ➞ 750
    dis(89, 20) ➞ 71.2
    dis(100, 75) ➞ 25
*/

fn find_the_discount(original: u64, discount: u8) -> f64 {
    // Substract discount from 100, then
    let dismul: u8 = 100u8 - discount;
    // multiply it by the original value and divide by a 100 
    (original as f64) * (dismul as f64) / 100f64;
} 

fn main() {
    println!("{}", find_the_discount(1500, 50));
    println!("{}", find_the_discount(89, 20));
    println!("{}", find_the_discount(100, 75));
}
