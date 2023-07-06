use std::collections::HashMap;

fn word_count(){
    let text = "hello are how hello are how";

    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1;    // count(reference) will own values at [word] key, and return it's value
    }
    println!("{:?}",map);
}
fn main(){
    let mut score = HashMap::new();
    score.insert(String::from("Blue"),10);

    let mut product : HashMap<String,u8> = HashMap::new();

    product.insert(String::from("Laptops"),10);
    product.insert(String::from("Mobile phones"),20);
    product.insert(String::from("Tablets"),4);

        // or_insert will return reference to value(of key) given inside it.
        // if key already exist, then return it's original value's reference, otherwise add key:value into hashmap
    let x = product.entry(String::from("Keyboard")).or_insert(13);
    println!("newly added value {}",x);
    for (key,value) in &product{
        println!("{} {}",key,value);
    };

    println!("\n");
    word_count();
}
