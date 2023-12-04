// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn displat_message(value: i32) {
    // if value < 5 {
    //     return println!("{:?}", "Value is less then 5");
    // }

    // if value > 5 {
    //     return println!("{:?}", "Value is greater then 5");
    // }

    // if value == 5 {
    //     return println!("{:?}", "Value is equal to 5");
    // }
    if value < 5 {
        println!("{:?}", "Value is less then 5");
    } else if value > 5 {
        println!("{:?}", "Value is greater then 5");
    } else {
        println!("{:?}", "Value is equal to 5");
    }
}

fn main() {
    displat_message(5);
}
