fn main() {
    {
        let s1 = String::from("hello");
        let s2 = s1;    // ownership is changed from s1 to s2, now s1 is no longer available
        // ! print ( s1 ) <- Error

    }   // s2 also goes out of scope and dropped, no longer available
        // ! print ( s2 )  <- Error

    let str1 = String::from("hello world");
    takes_ownership(str1);    // str1's value moved to the function, and no longer valid here
                                            // ! print ( str1 ) <- Error, who's str1 (cuz it goes out)

    let s1 = String::from("Zeshan Khalid");
                                                // s1 is gone
    let my_str = take_and_give(s1);  // firstly, my_str ownership will go to function,
                                                // function will also return ownership, will be reassigned to my_str


    let x = 5;
    makes_copy(x);      // value of x is just copied

    // what if function return some calculated ans value, how we will return ownership?
    // * using Tuple, we can return ans along with ownership
    let s2 = String::from("How are you?");
    let (greet,length) = calculate_length(s2);  // ownerShip will be return to greet
                                                                // value (size) will be return to length
}

    fn takes_ownership(some_str:String){
        println!("{some_str}");
    }   // some_str is dropped, and  goes out of scope

    fn makes_copy(some_val:i32){
        print!("value: {}",some_val);
    }   // only some_val (local var) goes out of scope, it's normal

    fn take_and_give(str:String) -> String{
        println!("name : {}",str);
        str // String will be returned to the Calling function (will return ownership)
    }

    fn calculate_length(s: String) -> (String,usize){ // will return both ownerShip and ans(size)
        let length = s.len();
        (s,length)
    }
