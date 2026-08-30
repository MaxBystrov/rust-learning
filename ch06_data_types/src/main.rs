fn main() {
converting_data_types();

floating_points_types();

numeric_operations();

boolean_types();

character_type();

tuple_type();

array_type();
}

fn converting_data_types() {
    let x = "49";
    let x: u32 = x.parse().expect("Not a number"); // parse() is for converting string into a number type
    let guess: u32 = "42".parse().expect("Not a number"); // expect() works with errors

    println!("x = {x}");
    println!("guess = {guess}");
}

fn floating_points_types() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.4 - 32.3;

    //multiplication
    let product = 4 * 7;

    // devision
    let quotient = 56.7 / 32.3;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 44 % 8;
}

fn boolean_types() {
    let t = true;
    
    let f: bool  = false;
}

fn character_type() {
    let c = "z"; 
    let z: char = 'Z'; // only ASCII 
    let emoji = "🎸";
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 32.5, 1);
    /*
    let (x, y, z) = tup;

    println!("The value of z is: {z}");
    */
    
    let five_hundred = tup.0;
    let thirty_two_point_five = tup.1;
    let one = tup.2;

    // the tuple without any values is unit
}

fn array_type() {
    let array = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let a: [i64; 8] = [1, 6, 2, 6, 9, 13, 15, 18];

    let b = [3; 5]; // 5 after semicolon gives us an array like this [3, 3, 3, 3, 3]

    let a_first = a[0];
    let a_last = a[7];
}
