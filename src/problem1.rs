use std::collections::HashMap;

fn main(){

    let input = vec![10,5,6,3,19,76,43,64,12,100,45,88,97, 21];
    println!("MEAN: {}",mean(&input));
    println!("MEAN :{}",mean_myself(&input));
    println!("Median: {}",median(&input));
    println!("Unsorted input is: {:?}",input);
    //mode(&input);
}

fn mean_myself(input:&Vec<i32>) -> f32 {
    let mut mean:f32 = 0.0;
    for i in input{
        let x = *i as f32 / input.len() as f32;
        mean += &x;
    }
    mean
}
fn mean(input:&[i32]) -> f32{
    input.iter().sum::<i32>() as f32 / input.len() as f32
}

fn median(sinput:&Vec<i32>) -> f32{

    //Sort the vector and find the middle element
    let mut input:Vec<i32> = sinput.to_vec();
    input.sort();
    println!("sorted input is: {:?}",input);
    match input.len()%2 {
        0 => {
            ((input[input.len()/2 - 1 ] + input[input.len()/2] ) / 2 )as f32
        }
        _ => input[input.len()/2] as f32
    }

}

fn mode(input:&[i32]){
    let mut map = HashMap::new();
     for numbers in input{
         let count = map.entry(numbers).or_insert(0);
         *count += 1;
     }
    println!("{:?}", map);

    /*
    let mut mode = HashMap::new();

    for (k,v) in map.iter(){
        if(mode.contains_key(&v)){
            mode.insert(v,mode.get(&v)+" "+ String::from(*k));
        }
    }
    println!("{:?}", mode);
    */
}


/*

fn sort(input:&[i32]){


}
fn sort_me(input:&[i32]){
    sort_me(input, 1, input.len());
}

fn sort_me(input:&[i32], p:i8 , r:i8){
    if(p<r){
        let q:i8 = (p+r)/2;
        sort_me(input,p,q);
        sort_me(input,q+1,r);
        merge_sort(input,p,q,r);
    }
}

fn merge_sort(input:&[i32], p:i8 , q:i8 , r:i8){
    let n = q-p+1;
    let m = r-q;

    let leftarray =

}

*/
