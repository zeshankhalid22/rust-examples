use std::num::ParseIntError;

fn multiply(first_num_str: &str, second_num_str: &str) -> Result<i32,ParseIntError> {
    match first_num_str.parse::<i32>(){
        // if first number parsed successfully, then check second
        Ok(first_num) => {
            match second_num_str.parse::<i32>() {
                Ok(second_num) => {
                    // return Ok(ans)
                    Ok(first_num * second_num)
                },
                // if second return Err
                Err(e) => Err(e),
            }
        }
        // if first return Err
        Err(e) => Err(e)
    }
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
