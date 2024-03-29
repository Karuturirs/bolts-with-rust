enum IpAddr{
    V4(String),
    V6(String),
}

 fn main(){
     let home = IpAddr::V4(String::from("127.0.0.1"));

     let loopback = IpAddr::V6(String::from("::1"));

     let some_number = Some(5);
     let some_string = Some("a string");
     let absent_number : Option<i32> = None;

     let x :i8 = 6;
     let y :Option<i8> = Some(5);
     println!("Adding x and y : {}", x+y);

 }
