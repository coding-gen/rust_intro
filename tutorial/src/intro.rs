use std::io;


fn f(x: u32) -> u32 {
    let y = if x < 5 {
        3 // this is the return of the block
    } else{
        2
    }; // note return types of blocks like this must be the same type
    y // return value
}

fn main() {
    println!("Hello, world!");
    let i: u32 = 5;
    // same as let i = 5u32;
    //    defined as the value being aassigned, as opped to defining the type of var
    // same as let i = 5_u32;
    // u8 - u128 // unsigned int 0-...
    // i8 - i128 // int -2^n/2 - 2^n/2
    {
        let i = i + 2;
        println!("int in the inner scope: {}", i);

    }
    println!("int: {}", i);

    let f: f32 = 10.9;
    let f2 = 12.7f32;
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


    // conversions
    let y = (i + 1) as u64; // cast using 'as'
    let y: u64 = (x + 1).into(); // type is defined at runtime. If y type changes, supported.

    // unit type
    let u: () = ();
    // it has one possible value, which is itself, the unit value
    println!("{:?}", u); // debug print. a modifier to the print format expression

    // block {} should end in an expression which determines the vlaue of that block
    // separator
    println!("{:?}", {/*empty expression*/;}); // prints a unit

    let mut i = 0;
    while i < 5 {
        println!("{}", i);



        match i {
            0 => println!("ready");
            1 => println!("steady");
            2 => println!("go");
            _ => panic!("bad i: {}", i); // all other values
            // bad_i => panic!("bad i: {}", i); // alt syntax 
            // pattern variable
            // don't set this to a var name you use elsewhere
        }
        i += 1;
    }
    println!("hello world!") // this is a constant, 
    // it is stored in static memory with program instructions.
    let s = "hello world!"; // this has type &str
    // reference to where "hello world!" is stored in mem
    // String type itself is an object with the value, length, and capacity (for dynamic length strings)
    // stores a pointer in the stack, which references where the dynamic sized variable is in the heap
    println!("{}", s); // fails because there isn't a format for a string.

    let s2 = String::from("hello"); // another syntax

}










