
#![allow(unused_variables)]
fn main() {
    use std::fmt::Display;
    //Generic Type Parameters, Trait Bounds, and Lifetimes Together
    println!("-->{}",
        longest_with_an_announcement(
                String::from("RAVI").as_str(),
                String::from("SANKAR").as_str(),
                5));

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
        where T: Display
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
    }
}
