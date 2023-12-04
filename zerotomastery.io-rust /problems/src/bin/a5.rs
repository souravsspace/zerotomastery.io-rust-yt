// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn show_numbers(mut value: i32) {
    loop {
        if value >= 100000 {
            println!("The Final Value: {}", value);
            break;
        }
        value += 1;
        println!("Ruuning: {}", value);
    }
}

fn main() {
    show_numbers(10)
}

// fn main() {
//     let mut an_number = 100;

//     loop {
//         if an_number >= 1000 {
//             println!("Final Value is: {}", an_number);
//             break;
//         }
//         an_number += 1;
//         println!("Value {}", an_number)
//     }
// }
