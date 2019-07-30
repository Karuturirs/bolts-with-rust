use std::fs;
use std::fs::File;
use std::io::Read;
use std::io;

fn main(){
    println!("{:?}",read_username_from_file());
    println!("{:?}",read_username_from_file_short());
    println!("{:?}",read_username_from_file_easy());
}

fn read_username_from_file() -> Result<String, io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
//read_to_string will automaticall open file and too
fn read_username_from_file_easy() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
