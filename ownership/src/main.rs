fn main() {
    // ownership start
    {
        let s = 0; // s valid
        println!("{s}"); // s valid
    }  // s not valid
    //s out of scope -> erro 
    //println!("{s}");


    //string push  
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");

    let s1 = s;
    println!("{s1}");
    //println!("{s}"); // error : value moved // to ensure memory safety rust make variable out of scope


    //clone/ copy
    let s = String::from("habib"); // can use s again as made out of scope previously
    let s_copy = s.clone(); // clonning different variable
    println!("{s} 's clone {s_copy}");


    // move 
    //contradicting concept of string (heap)
    //integer work like stack.
    let x = 5 ;
    let y = x ;

    println!("{x}");
    println!("{y}");

    //ownership/function
    takes_ownership(s);
    //println!("{s}"); -> error ownership gone -> 

    let x = 5;
    makes_copy(x);
    println!("{x}"); // no error integer works like stack 

    //return value/scope
    let s1 = gives_ownership();
    println!("{s1}");

    let (s2, len) = calculate_length(s1);
    println!("{s2} length {len}")

}

fn calculate_length(s:String) -> (String, usize){
    let length = s.len();
    return (s,length);
}

fn gives_ownership() -> String {
    let x = String::from("Ownership Given!");
    return x;
}

fn takes_ownership(some:String){
    println!("{} takes ownership", some);
}

fn makes_copy(inte:i32){
    println!("{inte}");
}

