fn main() {
    function_with_one_parameter(5); // we can call functions like this

    function_with_a_few_parameters(63, "James Hetfield");

    statement_and_expression();

    let x = function_with_return_values();
    println!("The value of x is: {x}");

    let b = function_with_return_values_and_parameters(5);
    println!("The value of b is: {b}");
}

fn function_with_one_parameter(x: i32) { // you have to write types (i32, u32, char, &str etc.)
    println!("The value of x is {x}");
}

fn function_with_a_few_parameters(age: i32, name: &str) {
    println!("{name} is {age} years old");
}

// Statement and expression

fn statement_and_expression() {
    // let y = 6; - it's a statement
    // fn () {} - it's also a statement
    // fn() - it's an expression

    // let x = (let y = 6); - you can't do like this
    
    let y = {
        let x = 3; // x no longer exists after expression block

        x + 1 // without a semicolon at the end
    };

    println!("The value of y is: {y}");
}

fn function_with_return_values() -> i32 { // write type like this
    32 // it's an expression so it's without a semicolon at the end
}

fn function_with_return_values_and_parameters(b: i32) -> i32 { // similar logic as functions with parameters
    b + 1
}