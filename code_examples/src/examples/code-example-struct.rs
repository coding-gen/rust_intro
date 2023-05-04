// struct notes
// from tutorials for structs and tuple structs:
// https://www.youtube.com/watch?v=lEbG8Q2Q1Ng&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=18
// https://www.youtube.com/watch?v=5z4CGI-uO9k&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=19

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// tuple struct
// no named members, just datatypes
// like a named tuple
struct ColorTuple(u8, u8, u8);

fn main() {
    // These are immutable.
    let bg = Color {
        red: 255,
        green: 70,
        blue: 15,
    };
    println!("{} {} {}", bg.red, bg.green, bg.blue);

    let bg2 = ColorTuple(255, 0, 0); // Just have to know what these stand for.
    println!("{} {} {}", bg2.0, bg2.1, bg2.2);
}