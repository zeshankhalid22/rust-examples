fn main () {
   let arr = vec![2,3,4,6];

    // inspect() is called on an iterator, and print based on condition

    // we can use inspect() with function chaining too
    // see output at every step

   let final_result:Vec<_> =
    arr.into_iter()
        .inspect(|x| println!("\nInit Val {}",x))
        .map(|x| x * 2)
        .inspect(|x| println!("After Doubling {}",x))
        .filter(|&x| x % 3==0)
        .inspect(|x| println!("after filter {}",x))
        .collect();

    println!("Result: {:?}",final_result);
}
