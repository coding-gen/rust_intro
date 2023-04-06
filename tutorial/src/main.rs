use std::io;

fn main() {
    println!("Hello, world!");
    let i: u32 = 5;
    // u8 - u128 // unsigned int 0-...
    // i8 - i128 // int -2^n/2 - 2^n/2
    {
        let i = i + 2;
        println!("int in the inner scope: {}", i);

    }
    println!("int: {}", i);

    let f: f32 = 10.9;
    // f32 // single precision float
    println!("float: {}", f);

    let t_or_f: bool = true;
    // let t:bool = 1; // actually this errs, look like it's not supported anymore.
    println!("boolean: {}", t_or_f);

    let c: char = 'b';
    println!("char: {}", c);


    let mut tup: (i32, bool, char) = (1, true, 's');
    println!("first tuple's bigint: {}", tup.0); // index

    let tup2: (i8, bool, char) = (1, true, 's');
    // tup = tup2; // fails because they are not the same type
    tup = (7, false, 't');

    // println!("{}", tup); // fails because there isn't a default print for this custom type
    println!("first tuple's modified bigint: {}", tup.0); // index
    println!("second tuple's smaller int: {}", tup2.0); // index

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // all elements must be same type, cannot resize
    // must initialize the full size with values
    println!("array: {}", arr[1]);


    // I/O
    let mut input = String::new(); // constructor
    println!("input a value please.");
    io::stdin().read_line(&mut input).expect("Failed to read line."); // reference
    println!("You entered: {}", input);

}

