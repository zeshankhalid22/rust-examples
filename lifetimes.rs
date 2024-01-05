fn func() {
    let y = 23;
    let mut r;
    {
        let x = 42;
        r = &x;
        println!("{}",*r);
    }
  // println!("{}",*r); // invalid here
    r = &y;
    println!("{}",*r);
}

fn main () {
    func();
}
