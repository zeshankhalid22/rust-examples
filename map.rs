/ * .map() can only be applied on iterators,
// to convert a collection into iterator, rust provides two methods,
// .iter() & .into_iter()
// return type of map is iterator.



fn main() {
    let nums = vec![2,5,7,8];
    let result: Vec<_> = nums.iter()
        .map(|n| {
            return n*10
        }).collect();
    println!("{:?}",result);
}
