use std::collections::HashMap;

fn main(){
    let mut score = HashMap::new();
    score.insert(String::from("Blue"),10);
    score.insert(String::from("Red"),30);

    println!("{:?}",score.get("Blue1"));

    // coverstion of vectors to HashMap
    let teams = vec![String::from("Blue"),String::from("Red")];
    let scores1 = vec![10,20];

    let teams_map: HashMap<_,_> = teams.iter().zip(scores1.iter()).collect();
    //println!("{:?}",teams_map.get("Blue"));

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}",map.get("Favorite color"));
    let key = String::from("Favorite color");
    println!("{:?}",map.get(&key));

    for (k,v) in &score{
        println!("{}:{}",k,v);
    }

    println!("{:?}",score.get("Blue"));

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{:?}", map);
    }

    println!("{:?}", map);

}
