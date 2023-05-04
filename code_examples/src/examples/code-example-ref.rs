// pass by reference notes
// from tutorials 16 (references) and 19 (pass by reference)
// https://www.youtube.com/watch?v=bwKITNsaPVk&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=19

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    // standard reference
    let mut x = 10;
    let xr = &x;
    let xrr = &x;
    println!("x is: {}", xr);

    // since xr is only a reference, we can still use x.
    println!("x is: {}", x);

    // multiple references ok.
    println!("x is: {}", xrr);

    {
        // Define a scope for the mutable ref to x.
        // When the scope ends, the borrowing of x ends, and x can be used again.

        // a mutable reference.
        let mutable_xr = &mut x;

        // To mutate it, must dereference the reference.
        *mutable_xr += 1;
        // println!("x is: {}", x) // This cannot be used in the same scope as mutable_xr.
    }
    println!("x is: {}", x); // This is fine because it is outside the scope of the mutable ref to x.

    // These are immutable.
    let blue = Color {
        red: 0,
        green: 0,
        blue: 255,
    };
    print_color(&blue);

    // Can use again, since we only passed a reference, not the whole blue object.
    print_color(&blue);
}

// & is reference
fn print_color(c: &Color) {
    println!("Color: red: {} gree: {} blue: {}", c.red, c.green, c.blue);
}
