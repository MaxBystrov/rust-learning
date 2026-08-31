// nothing hard
fn main() {
    if_expression();

    multiple_conditions();

    using_if_in_a_let_statement();

    repetition_with_loops();

    loop_labels();

    loops_with_while();

    loops_with_for();
}

fn if_expression() {
    let number = 5;

    if number < 4 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn multiple_conditions() {
    let number = 9;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn using_if_in_a_let_statement() {
    let condition = true;
    // you can't ... { 5 } else { "ten" }
    let result = if condition { 5 } else { 10 }; 
    
    println!("Result: {}", result);
}

fn repetition_with_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {}", count);
}

fn loops_with_while() { // it's more safe to use for loops, but while loops are still useful
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        
        index += 1;
    }
}

fn loops_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..=4).rev() {
        println!("{number}!");
    }
}