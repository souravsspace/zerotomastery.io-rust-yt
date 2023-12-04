// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn is_admin(string_value: &str) -> bool {
    match string_value {
        "Joy" => false,
        "Rue" => false,
        "Sourav" => true,
        _ => false,
    }
}

fn guess_admin(string_value: &str) -> &str {
    match string_value {
        "Akhi" => "No, it's not Akhi",
        "Joy" => "No, it's not Joy",
        "Sourav" => "Yes, it's Sourav",
        _ => "No, it's not an admin name",
    }
}

fn main() {
    const ADMIN_NAME: &str = "Sourav";

    let is_admin = is_admin(ADMIN_NAME);

    if is_admin {
        let admin_name = guess_admin(ADMIN_NAME);
        println!("{}", admin_name);
    } else {
        println!("{} is not an admin", ADMIN_NAME);
    }
}
