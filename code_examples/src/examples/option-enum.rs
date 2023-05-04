// Option (enum) notes
// from tutorial:
// https://www.youtube.com/watch?v=JKmkKae-EhM&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=37
// Option can be a value, or no value. To handle function returns.

fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Dominic" => Some("Software Dev"),
        "Coding Gen" => Some("Student"),
        _ => None,
    }
}

fn main() {
    let name = String::from("George");

    // This returns an option.
    // Either Some/None depending on the length of the string.
    println!(
        "{}",
        match name.chars().nth(8) {
            // Handle the cases with a switch
            Some(c) => c.to_string(),
            None => "No char at index 8!".to_string(),
        }
    );

    println!(
        "Occupation is: {}",
        match get_occupation("Salamander") {
            Some(o) => o,
            None => "No occupation found.",
        }
    );
}