fn loop_implementation(){
    // * Actual Implementation of loop
    // i holds the reference to 1st element in the vector
    let mut i = vec!["a","b","c"].into_iter();

    // while Some(val) is not "None", keep moving and printing
    while let Some(val) = i.next(){
        println!("{}",val);
    }
}

fn map_examples(){
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // * 1.
    //  a new iterator that applies transformation to each element in the number
    let double = numbers.iter().map(|x|  x * 2);
    for x in double {
        print!("{} ",x);
    }   // now we cannot use double, it is consumed
    println!();

    // * 2.
    let mut count = 0;
    for pair in vec!["Zeshan","Ahmed","Ali"].into_iter()
        .map(|single_name| {
            count += 1;
            (count,single_name)
        })
    {   // loop body starts here
        print!("{:?} ",pair);
    }
    // vector elements goes to single_name and returned back to pair(_,_)

}

fn enumerate_example(){
    let numbers = vec![2,4,6,8];
    let even_indexes = numbers
        .iter() // reference to numbers
        .enumerate()    // returns (index,value) of type (usize,i32)
        .filter(|&(index,_)| index%2==0) // returns Iterator to even indexes
        .map(|(_,value)| value); // as filter returned (index,value), we finally only want "values", so ignore indexes

    for even in even_indexes{
        print!("{} ",even);
    }   println!();

}

fn main(){
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // * Uncomment the desired code to run

    // loop_implementation();
    // map_examples();
    // enumerate_example()


    // let mut iter_mut = numbers.iter_mut();  // mutable iterator can change values
    // for num in iter_mut{
    //     *num +=1;
    // }

   //  // * filter()
   // let greater_then_5 = numbers.iter.filter(|x| **x >= 5);
   //  for x in greater_then_5{
   //      print!("{} ",x);   // 5 6 7 8
   //  }   println!();

    // * any()
    let list = vec![3, 1, 5, -2];
    let check = numbers.iter().any(|&x| x > 0);
    println!("does list has a negative value? {}",check);

}
