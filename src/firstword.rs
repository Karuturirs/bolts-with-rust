
fn main(){

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let x = String::from("string slice");

    println!(first_word(x));
}

fn first_word(s : &String) -> &str{
    let bytes = s.as_bytes();

    for(i , &item ) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    &s[..]

}
