fn main() {
  let greetings: Option<String> = Some(String::from("hi there"));

  match greetings {
    // * Value will partially move to msg
       // Some(msg) => println!("{}", msg),
    // borrow(with ref) instead of move
    Some(ref msg) => println!("{}",msg),
    None => println!("Nothing to greet"),
  }

  println!("Greetings? {:?}", greetings);

