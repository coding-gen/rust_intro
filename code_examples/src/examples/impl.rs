// impl notes
// from tutorial:
// https://www.youtube.com/watch?v=brrVYgRV7os&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=21
// An impl is for adding methods to a struct

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // functions for the struct go here
    fn print_description(&self) {
        println!("Rectangle: {} x {}.", self.width, self.height);
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let my_rect = Rectangle {
        width: 10,
        height: 10,
    };

    my_rect.print_description();
    println!(
        "The rectangle is{} a square.",
        if my_rect.is_square() { "" } else { " not" }
    );
}
