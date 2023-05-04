// Vec notes
// from tutorials for vectors:
// https://www.youtube.com/watch?v=GcsAQTMYR1M&list=PLVvjrrRCBy2JSHf9tGxGKJ-bYAN_uDCUL&index=24
// Vector is a powerful array.

fn main() {
    // Type must be defined in the <arows>
    let mut my_vector: Vec<bool> = Vec::new();

    my_vector.push(true);
    my_vector.push(false);
    my_vector.push(true);
    my_vector.push(false);
    my_vector.push(true);
    my_vector.remove(1);
    println!("{:?}", my_vector);

    let int_vector = vec![1, 3, 5, 7];
    println!("{}", int_vector[2]);

    for (i, element) in int_vector.iter().enumerate() {
        if my_vector[i] {
            println!("{}", element);
        }
    }
}