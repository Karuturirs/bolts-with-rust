use std::io;


fn main(){
    println!("Welcome to Math world");
    println!("Find Fibonacci element at :");

    let mut nthelement = String::new();

    io::stdin().read_line(&mut nthelement)
            .expect("Failed to read the input");

    let nthelement :u32 = nthelement.trim().parse()
            .expect("Not a number");

        println!("{}", fib(nthelement));
}

fn fib(i:u32) -> u32 {
    match i{
        0 => 1,
        1 => 1,
        _ => fib(i-1) + fib(i-2),
    }
}
