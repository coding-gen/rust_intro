// traits notes
// from tutorials for implementing and defining traits:
// https://www.youtube.com/watch?v=qHnVtb1qHR0&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=23
// https://www.youtube.com/watch?v=0sI-GzVSYic&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=28
// A trait is like an interface. Something a object/class/struct can do.
// Can be a set of rules or requirements for an object/class/struct.

struct Person {
    name: String,
    age: u8,
}

// In order to have this trait, the following must be true.
trait HasVoiceBox {
    // If you have this trait, then:

    // You have the ability/function to speak.
    fn speak(&self);

    // But it's possible it's not mastered yet, so it comes with a check.
    fn can_speak(&self) -> bool;
}

// The to_string trait has 1 method ToString
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!(
            "The person's name is {} and they are {}.",
            self.name, self.age
        );
    }
}

impl HasVoiceBox for Person {
    // Similar to class or header inheritance.
    // Here, fill in the traits for this Person struct with HasVoiceBox trait.
    fn speak(&self) {
        println!("Howdy there!");
    }

    fn can_speak(&self) -> bool {
        if self.age > 2 {
            return true;
        }
        false
    }
}

fn main() {
    let coding_gen = Person {
        name: String::from("Coding Gen"),
        age: 57,
    };
    println!("{}", coding_gen.to_string());
    if coding_gen.can_speak() {
        println!(
            "{} is over 2, so we'll let them speak for themselves.",
            coding_gen.name
        );
        coding_gen.speak();
    }
}