use std::io;
use console::Term;
fn main() {
    loop{
        println!("What do you wanna do?");
        println!("+\t-\t*\t/");

        let mut sign:char;
        io::console()
            .read_char(&mut sign)
            .expect("Please enter sign");
            
        if sign == '+' {
            sum();
        }
        else if (sign == '-') {
            sub();
        }
    }
}
fn sum(){
    println!("Sum Function");
}
fn sub(){
    println!("Substract Function");
}
fn mul(){
    println!("Multiply Function");
}
fn div(){
    println!("Divide Function");
}