use std::io;
use std::process::exit;
use colored::Colorize;

fn main() {
    println!("{}", "Welcome to basic calculator".yellow());
    println!("{}", "*###########################*".yellow());

    loop {
        select_operation();
    }
}

fn select_operation() {
    println!("Sum = 1\nSub = 2\nMultiple = 3\nDivider = 4\nExit = q");
    println!("{}", "***************************".yellow());
    println!("{}", "Please select operation: ".yellow());

    let mut selected_operation = String::new();

    io::stdin()
        .read_line(&mut selected_operation)
        .expect("Error Please Select Operation");

    selected_value(selected_operation);
}

fn selected_value(selected: String) {
    if selected.contains("q") {
        println!("{}", "You pressed the 'q' Key. The program is closing...".red());
        exit(1);
    }
    let options = vec!["1", "2", "3", "4"];


    if options.iter().any(|o| selected.contains(o)) {
        println!("{}", "Number One: ".blue());
        let mut num_one_in = String::new();

        io::stdin()
            .read_line(&mut num_one_in)
            .expect("Error Please Enter Number One");

        println!("{}", "Number Two: ".blue());
        let mut num_two_in = String::new();

        io::stdin()
            .read_line(&mut num_two_in)
            .expect("Error Please Enter Number One");

        let num_one: f32 = num_one_in.trim().parse().expect("Not a valid number");
        let num_two: f32 = num_two_in.trim().parse().expect("Not a valid number");

        if selected.contains("1") {
            sum(num_one, num_two);
        } else if selected.contains("2") {
            sub(num_one, num_two);
        } else if selected.contains("3") {
            multi(num_one, num_two);
        } else if selected.contains("4") {
            div(num_one, num_two);
        }
    } else {
        println!("{}", "You made an incorrect selection. Please try again.".red());
    }
}

fn sum(num_one: f32, num_two: f32) {
    let result = num_one + num_two;
    println!("{}", "***************************".yellow());
    println!("{} {}", "Sum Result: ".blue(), result.to_string().green());
    println!("{}", "***************************".yellow());
}

fn sub(num_one: f32, num_two: f32) {
    let result = num_one - num_two;
    println!("{}", "***************************".yellow());
    println!("{} {}", "Sub Result: ".blue(), result.to_string().green());
    println!("{}", "***************************".yellow());
}

fn multi(num_one: f32, num_two: f32) {
    let result = num_one * num_two;
    println!("{}", "***************************".yellow());
    println!("{} {}", "Multiple Result: ".blue(), result.to_string().green());
    println!("{}", "***************************".yellow());
}

fn div(num_one: f32, num_two: f32) {
    let result = num_one / num_two;
    println!("{}", "***************************".yellow());
    println!("{} {}", "Dividercleq\
     Result: ".blue(), result.to_string().green());
    println!("{}", "***************************".yellow());
}