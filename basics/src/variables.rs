pub fn run() {
    //String variable
    let string = String::from("This is a string");

    println!("{}", string);

    //Intergers
    let number: i32 = 30;
    println!("This is a 32-bit integer: {}", number);

    //Tuples
    let tuple = (12, 12, String::from("Tuple value 3"));
    println!("The first value of the tuple is {}", tuple.0);

    //Array 
    let array: [i32; 4] = [10, 20, 30, 40];
    println!("The second value is {}", array[1]);
    println!("The length of the array is {}", array.len());
}