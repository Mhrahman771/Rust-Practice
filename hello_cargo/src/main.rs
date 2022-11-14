use std::io;
use std::cmp::Ordering;

use colored::Colorize;
use rand::Rng;


fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=101);

    

    loop{
        println!("The secret number is {}", secret_number);
        println!("Enter a number to guess...");

        let mut guess  = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Get input !!");

        println!("You wrote {}", guess);
        println!("The secret number is {}", secret_number);


        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Integer guess {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Greater => println!("{}", "Too Big".red()),
            Ordering::Equal => {
                println!("{}", "You WIN!".green());
                break;
            }
        };
    }
}
