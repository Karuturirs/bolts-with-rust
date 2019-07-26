
#![allow(unused_variables)]
fn main() {
    let data = "initial contents";

    let s = data.to_string();
    println!("{}",s);
    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    //push_str doesnot take the owner ship
    println!("s2 is {}", s2);
    // push just takes single character
    s1.push('l');
    println!("s1 is {}", s1);

    // Concatenation with the + Operator or the format! Macro

    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4; // note s3 has been moved here and can no longer be used, syntax for fn add(self, s: &str) -> String {
    println!("s5 is {}", s5);


    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let stt = tic + "-" + &tac + "-" + &toe;
    println!("stt is {}", stt);
    //usage of format
    let s6 = String::from("tic");
    let s7 = String::from("tac");
    let s8 = String::from("toe");
    // no owner ship is taken by using format macro
    let s9 = format!("{}-{}-{}", s6, s7, s8);
    println!("s9 is {}", s9);

    let hello = "Здравствуйте";

    // length hello is not 12 but 24 as the string is encoded in UTF8 taked much long to get the characters
    println!("{}",hello.len());

    let answer = &hello[0..4];

    println!("{}",answer);
    // iterating on string using chars()
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //using byte method will return as_bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}
