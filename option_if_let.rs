use std::fmt::Debug;
use std::mem::align_of_val;

fn main(){
    let my_vec =  vec![1, 3, 5];

    // if function returns Some<T>, print it
    if let Some(res)  = my_vec.get(2){
        println!("Element 2 : {res}");
    }

    if let Some(res) = my_vec.get(232){
        // if returns None, don't reach here, neither panic
        println!("index not found");
    }


    let temp = -6;
    // var1 has Option<> data type, which will store either Some() or None
    let var1 = if temp>=0{
        Some(temp)
    }
    else { None };

    // if value has Some<> variant, print statement
   if var1.is_some(){
       println!("var1  has Some<> Variant");
   }
    // i can use else Statement too
    // else { println!("var1 has None variant"); }

    // if var1 has None variant
    if var1.is_none(){
        println!("var1  has None Variant");
    }

    let text = "hello rustaceans, how are you";

    // .find return Option<usize> (index where value found), which then be stored in Some. usize will be assign to U
    if let Some(U) = text.find("ello") {
        println!("if let Some = {}",U);
    }
}
