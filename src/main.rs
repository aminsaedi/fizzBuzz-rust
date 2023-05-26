fn main() {
    print_fizz_buzz(100);
}

fn print_fizz_buzz(till: i8) {
    for curr in 1..=till {
        println!("{}", print_number_fizzbuzz_status(curr));
    }
    println!("");
}

/// Receives a number and return a strig to represent it's fizz buzz staus
/// # Args
/// ## num: i8
/// The input number being checked
/// # Retrurn type
/// ## String
fn print_number_fizzbuzz_status(num: i8) -> String {
    let fizz: bool = is_fizz(num);
    let buzz: bool = is_buzz(num);

    if fizz & buzz {
        format!("  FizzBuzz  ")
    } else if fizz {
        format!("  Fizz  ")
    } else if buzz {
        format!("  Buzz  ")
    } else {
        format!("  {num:?} ")
    }
}

fn is_fizz(num: i8) -> bool {
    num % 3 == 0
}

fn is_buzz(num: i8) -> bool {
    num % 5 == 0
}
