
// Variables

/*
fn main() {
    //by default immutable variable
    let x = 5;
    println!("The Value is {}", x);

    let x = "six"; 
    println!("The duplicate value is {}", x);


    // can't mut constant
    const SOURCE: u32 = 1000;
    println!("The constant is {}", SOURCE);
    // shadowing -> creating new variable with existing name

    // data type scalar -> single value
    // compound -> a group of values

    //4 min scalar data types - integers -float -booleans -character
    /* */
    let dec = 98_222; //decimal
    let hex = 0xff; //hex
    let oct = 0o77; //octal 
    let bin = 0b1111_0000; //binary
    let byte = b'A'; //Byte (u8 only)

    println!("Decimal {dec}, Hexadecimal {hex}, Octal {oct}, Binary {bin}, Byte {byte}")

    loop{
        let mut x = String::new();
        println!("Enter x: ");
        io::stdin()
            .read_line(&mut x)
            .expect("Retype");

        let mut y= String::new();
        println!("Enter y: ");
        io::stdin()
            .read_line(&mut y)
            .expect("Retype");

        let n1: u32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let n2: u32= match y.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("x: {}, y: {}", n1, n2);

        let main_sum = sum (n1, n2);
        println!("The main function sum = {}", main_sum);
        break;
    }

}


fn sum(x: u32, y: u32) -> u32{
    println!("First Number : {}, Second Number: {}", x, y);

    let sum: u32 ;
    sum = x + y;

    println!("Sum x + y = {}", sum);
    return sum;
}


fn main(){

    let mut x = 5 ;
    println!("x : {}", x);

    x = 6;
    println!("Again x : {x}");

    let x = x + 1;
    println!("x+1 : {x}");

    {
        // scope variable 
        let x = x * 11;
        println!("x*11: {x}");
    }

    //shadowing x 
    println!("Final x: {x}");

    // not allowed to mutate variable type
    let spaces = "   ";
    //spaces = spaces.len();
    println!("Spaces: {spaces}");
}
*/

//Data Types

//scalar type
/*
fn main(){
// signed -> -(2^n-1 to 2^n-1 -1)
// i8 -> 2^7 to 2^7 - 1 -> -128 to 127
// arch -> isize, usize -> for 64 bit architecture decler 64 bit 
// for 32 bit architecture decler 32 bit variable
// _ as separator 1_000 like , in cash count 1,000 (thofufsand)
// default i32 
    let x = 3;
    println!("Integer x: {}", x);

    //Float f32/f64
    //deafult f 64 -> modern CPUs roughly same speed as f32
    //floating point types are signed
    //f32 -> single precision & f64 -> double precision

    let mut x = 2.0; //default
    let y: f32 = 5.0;
    println!("Flat x: {x}");
    println!("Flat Y: {y}");

    x = x * 2.25;
    println!("Mul x: {x}");

    //boolean type

    let bool = true;
    println!("Boolean B: {}", bool);


    // character type

}
*/

//compound type
/*
fn main(){
    //tuple type
    let tup = (400, 4.5, 1, "hello");
    //let (x, y, z, a) = tup;
    //println!("Tuples Type: {} {} {} {}", x, y, z, a);
    println!("x: {}, y: {}, z: {}, a: {}", tup.0, tup.1, tup.2, tup.3);

    let x = tup.3;
    println!("x: {x}");

    //array type
    
    let a:[u32; 5] = [1, 2, 3, 4, 5];
    let months = ["January", "February"];
    println!("a.0 = {}", a[0]);
    println!("months[0] = {}", months[1]);
}
*/
//index + array
/*
fn main(){
    let a = [1, 2, 3, 4, 5];

    println!("Enter array Index: ");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line!");

    let index: usize = index
        .trim()
        .parse()
        .expect("Entered not a number!");
    
    let element = a[index];
    println!("Index: {index}, Element: {element}");


    let y ={
        let x = 3;
        x + 1
    };
    println!(" y: {}",  y);

}
*/


//control statement



fn main(){
    //while loop
    let a =[11, 12, 13, 14, 15];
    //let mut index = 0;

    for number in (1..=5).rev() {
        println!("{number}");
    }
}
