fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    constants();

    shadowing();

    reusing_the_name();
}

fn constants() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // You should type it with caps
    
    println!("{THREE_HOURS_IN_SECONDS}"); // print the value using formating (not only for constants)
}

fn shadowing() {
    let x = 5;

    let x = x + 1;
    
    {   // after this scope x will be the same as before it
        let x = x * 2;
        println!("The value of x in inner scopes is: {x}");
    }

    println!("The value of x is: {x}");
}

fn reusing_the_name() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");

    // you can't do like this:
    // let mut spaces = "   ";
    // spaces = spaces.len();
    
}