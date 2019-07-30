use std::fs::File;

fn main() {
    //using unwrap method of result enum which returns Ok or panic for the error
    let f = File::open("hello.txt").unwrap();

    //using except method
    let f = File::open("hello.txt").except("Failed to open hello.txt");

}
