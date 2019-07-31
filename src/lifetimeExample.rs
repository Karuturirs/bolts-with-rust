
#![allow(unused_variables)]
fn main() {

    //this reference can live for the entire duration of the program.
    let s: &'static str = "I have a static lifetime.";

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Lifetime Annotations in Struct Definitions
    struct ImportantExcerpt <'a>{
        part: &'a str,
    }


    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };


}
