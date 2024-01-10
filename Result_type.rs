use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32,ParseIntError> {
    // and_then() takes Result, if Err then return error
    // else if Ok(val), then takes val and return another Ok(val)
    first_num_str.parse::<i32>().and_then(|first_num|{
        // map takes val of Ok(val), and return val * x or whatever
        second_num_str.parse::<i32>().map(|second_num| {
            first_num * second_num
        })
    })
}

// custom print function to display Result<Ok,Err>
fn print(result: Result<i32,ParseIntError>) {
    match result {
        Ok(num) => println!("n is {}",num),
        Err(e) => println!("Err: {}",e),
    }
}

fn main () {
    let var1 = multiply("10","13");
    // custom print function
    print(var1);

    // checking for invalid int
    let var2 = multiply("23a","42");
    print(var2);
}
