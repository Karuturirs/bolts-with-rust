#[derive(Debug)]
struct Rectangle{
    length:u32,
    width :u32
}

impl Rectangle{
    fn area(&self)->u32{
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle)-> bool {
        self.length > other.length  && self.width > other.width
    }
}

fn main(){

    let rec1 = Rectangle{
        length: 5,
        width : 7
    };
    println!("rect1 is {:#?}", rec1);
    println!("Area of rectangle using function is : {}", area_rec(&rec1));
    println!("Area of rectangle using method is : {}", rec1.area());

    let rec2 = Rectangle{
        length: 3,
        width : 2
    };

    println!("can rec1 hold rec2: {}", rec1.can_hold(&rec2));
}

fn area_rec(rectangle: &Rectangle)->u32{
  rectangle.length * rectangle.width
}
