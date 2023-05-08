/* 
Create a function that takes: 
    * the height and 
    * the radius of a cone 
as arguments and returns: 
    * the volume of the cone rounded to the nearest hundredth. 

Examples:
    * cone_volume(3, 2) ➞ 12.57
    * cone_volume(15, 6) ➞ 565.49
    * cone_volume(18, 0) ➞ 0

Notes:
    * Return approximate answer by rounding the answer to the nearest hundredth.
    * Use Python's math.pi constant or equivalent, don't fall for 3.14 ;-)
    * If the cone has no volume, return 0.
*/

const PI: f64 = std::f64::consts::PI;

fn cone_volume(height: u32, radius: u32) -> f32 {
    // Do the math (in f32)
    let volume: f32 = (1f32/3f32) * (PI as f32) * (radius as f32) * (radius as f32) * (height as f32);

    // Print
    println!("{}", volume);

    // Return
    return volume;
}

fn main() {
    cone_volume(3, 2);
    cone_volume(15, 6);
    cone_volume(18, 0);
}
