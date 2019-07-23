use std::io;

fn main(){

    println!("Welcome to temperature covertor..!");
    println!("Happy to help you today, what do you need?");
    println!("1.Convert Fahrenheit to Celsius");
    println!("2.Convert Celsius to Fahrenheit");

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
                .expect("Failed to read your input");
    let choice: u8 = choice.trim().parse()
            .expect("Failed to parse input");
    if choice == 1 {
        println!("--> {} C",tempto_c(take_input_temp('F')));
    }else{
        println!("--> {} F",tempto_f(take_input_temp('C')));
    }

}


fn tempto_f(x:f32)->f32{
    let tf: f32 = (x * 1.8 )+ 32.0;
    tf
}

fn tempto_c(x:f32)->f32{
    let tc: f32 = (x - 32.0) * 0.5556;
    tc
}

fn take_input_temp(y:char) -> f32 {
    println!("Enter the {} temperature : ",y);
    let mut inputtemp = String::new();
    io::stdin().read_line(&mut inputtemp)
        .expect("Failed to read your input temperature");

    let inputtemp: f32 =  inputtemp.trim().parse()
                        .expect("Failed to parse input temperature");

    inputtemp
}
