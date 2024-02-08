macro_rules! my_vec {
    // , = one or more separated things
    // ($element:expr) is the pattern where $element matches any expression and
    // gives the founded expression the name element

    // Block 1
    ($ ($element:expr),* ) => {
        // ($elem:expr,) if commas are at the end of elem like [1,] [1,2,3,]
        {
        let mut vec = Vec::new();
        // $(___)* to give repetitions currosponding to the pattern
        $(
            vec.push($element);
        )*
        vec
         }
    };

    // Block # 2 takes [element; count_of_how_many_times_to_repeat/push]
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::with_capacity($count);
        for _ in 0..$count {
            vs.push($element);
        }
        // vs.resize($count, $element) // also work
        vs
    }};
}


fn main() {

    let mv = my_vec!(1,3,6);
    println!("{:?}", mv);
    let mv2 = my_vec!(2;5);
    println!("{:?}", mv2);
}
