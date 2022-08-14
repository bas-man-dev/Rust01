use std::io;

fn main(){

    println!("Enter any number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    println!("Entered number is {}", number);

}
