use std::fs::File;
use std::io::ErrorKind;
use crate::Errorhandling;

fn main(){
    let f = File::open("hello.txt");

   let f = match f {
       Ok(file) => file,
       Err(error) => match error.kind() {
           ErrorKind::NotFound => match File::create("hello.txt") {
               Ok(fc) => fc,
               Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
           },
           other_error => panic!("There was a problem opening the file: {:?}", other_error),
       },
   };
   Errorhandling::otherway();
}
mod Errorhandling{
    pub fn otherway(){
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("Tried to create file but there was a problem: {:?}", error);
                })
            } else {
                panic!("There was a problem opening the file: {:?}", error);
            }
        });
    }
}
