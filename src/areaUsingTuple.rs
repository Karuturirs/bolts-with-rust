
fn main(){

    let rec1 = (5,7);
    println!("Area of rectangle is : {}", area_rec(rec1));

}

fn area_rec(dimentions:(u32, u32)) -> u32{
    dimentions.0 * dimentions.1
}
