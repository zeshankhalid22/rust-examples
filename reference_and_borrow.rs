
fn main() {
    let mut s1 = String::from("Hello");

    let length: usize = calculate_length(&s1);  // let's refer to s1, borrow
    println!("length of {} is {}.",s1,length);

    change(&mut s1);    // we allowed reference to change the value
    println!("updated value of s1 {}",s1);

    let r1 = & mut s1;
    // ! we cannot use two mutable reference to a single variable
    // let r2 = & mut s1;

    println!("{r1}");   // r1 goes out of scope and won't use used,
    let r2 = & mut s1;  // so this is valid now
    println!("{r2}");
}

fn calculate_length(s: &String) -> usize{
    s.len()
}   // bcz s does not have ownership it refers to, it does not drop anything

fn change(str: &mut String){
    str.push_str(", world");
}
