 use std::io::{self, Write};
 
fn main(){
   let mut v = vec![100,2,5,6,];

   for i in &mut v {
       *i += 50;
   }
   for i in &v {
       println!("{}",i);
   }

   #[derive(Debug)]
   enum SpreadsheetCell{
       Int(i32),
       Float(f64),
       Text(String),
   }

   let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
   ];


   for i in &row{
       println!("{:?}", i);
   }


   let buffer = vec![1, 2, 3, 5, 8];
   io::sink().write(buffer.as_slice()).unwrap();

}
