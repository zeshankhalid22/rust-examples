
macro_rules! my_macro {
    // , = one or more separated things
    // ($element:expr) is the pattern where $element matches any expression and
    // gives the founded expression the name element

    ($ ($element:expr),* ) => {
        {
        let mut vec = Vec::new();
        // $(___)* to give repetitions currosponding to the pattern
        $(
            vec.push($element);
        )*
        vec
         }
    }

}


fn main() {
    let mv = my_macro!(1,3,6);
    println!("{:?}", mv);
}
