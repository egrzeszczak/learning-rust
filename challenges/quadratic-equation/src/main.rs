/*
A quadratic equation a x² + b x + c = 0 has either 0, 1, or 2 distinct solutions for real values of x. 

Given a, b and c, you should return the number of solutions to the equation.

Examples:
    solutions(1, 0, -1) ➞ 2
        x² - 1 = 0 has two solutions (x = 1 and x = -1).
    
    solutions(1, 0, 0) ➞ 1
        x² = 0 has one solution (x = 0).
    
    solutions(1, 0, 1) ➞ 0
        x² + 1 = 0 has no real solutions.
*/

fn solutions(a: i32, b: i32, c: i32) -> u8 {
    // delta = b^2 - 4ac
    let delta: f32 = ((b*b) as f32) - 4.0 * (a as f32) * (c as f32);

    println!("a='{}' b='{}' c='{}' delta='{}'", a, b, c, delta);

    if delta > 0f32 {
        return 2u8;
    } else if delta == 0f32 {
        return 1u8;
    } else {
        return 0u8;
    }
}

fn main() {
    solutions(1, 0, -1);
    solutions(1, 0, 0);
    solutions(1, 0, 1);
}
