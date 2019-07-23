
fn main(){
    let length = 5;
    let width = 7;

    println!("Area of rectangle is : {}", area_rec(length, width));

}

fn area_rec(length:u32 , width:u32 ) -> u32{
    length * width
}
