/*

Jazify:
Create a function which concatenates the number 7 to the end of every chord in a list.
Ignore all chords which already end with 7.

Examples:
    * jazzify(["G", "F", "C"]) ➞ ["G7", "F7", "C7"]
    * jazzify(["Dm", "G", "E", "A"]) ➞ ["Dm7", "G7", "E7", "A7"]
    * jazzify(["F7", "E7", "A7", "Ab7", "Gm7", "C7"]) ➞ ["F7", "E7", "A7", "Ab7", "Gm7", "C7"]
    * jazzify([]) ➞ []

Notes
    * Return an empty list if the given list is empty.
    * You can expect all the tests to have valid chords.

*/

fn jazzify(mut chords: Vec<&str>) -> Vec<String> {
    
    // Initialize new vector with String type due to str being inmutable
    let mut result: Vec<String> = vec![];
    
    // Print values before jazzification
    println!("Before jazzification: {:#?}", chords);
    
    // Jazzification
    for chord in chords.iter_mut() {
        if !chord.ends_with("7") {
            result.push(format!("{}{}",*chord,String::from("7")));
        } else {
            result.push(String::from(*chord));
        }
    }

    // Print values after jazzification
    println!("After jazzification: {:#?}", result);

    // Return result
    return result;
}

fn main() {
    jazzify(vec!["G", "F", "C"]);
    jazzify(vec!["Dm", "G", "E", "A"]);
    jazzify(vec!["F7", "E7", "A7", "Ab7", "Gm7", "C7"]);
    jazzify(vec![]);
}
