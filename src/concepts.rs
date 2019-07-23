
fn concepts(){
    let x = 10; //immutable
    let mut y = 20; // muttable
    let z:i32  = 50; //signed integre
    let a:f32 = 4.3;
    let flag:bool = true ;//  bool takes 1 byte
    let c = 'Z'; // char takes 4 bytes, so can store more than just char
    const AAA_BBB:u32 = 657 ; //conustant

    //shawdoing
    let x =x+1;

    // tuple is fixed but can be mixed
    let tup :(i32, f64, u8, bool)=(5000,6.4,1,false);

    let ( a, b, c, d ) = tup;

    println!("the value of b is {}",b);

    println!("value of tup at 0 is: {}",tup.0);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let index = 10;

    let element = a[2];

    println!("the value fo element is :{}", element);
    another_function(a[0]);
}

fn another_function(x:i32){
    println!("The value of x is: {}", x);
}
