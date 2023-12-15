#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn into_iter() {
    let mut points = vec![Point { x: 1.1, y: 1.2 }, Point { x: 2.2, y: 2.3 }];

    // into_iter() takes ownership of points // by value
    // let  p1: Point = points.into_iter().next().unwrap();

    // by reference to points
    let p3 = points.iter().next().unwrap(); // (&points).into_iter().next().unwrap();
    println!("{:?}", p3);
    // by mut reference to points
    let p4 = points.iter_mut().next().unwrap(); // (&mut points).into_iter().next().unwrap();
    p4.y = 11.2;
    p4.x = 4.2;
    println!("{:?}", p4);
}

fn loop_implementation() {
    // * Actual Implementation of loop
    // i holds the reference to 1st element in the vector
    let mut i = vec!["a", "b", "c"].into_iter();

    // while Some(val) is not "None", keep moving and printing
    while let Some(val) = i.next() {
        println!("{}", val);
    }
}

fn map_examples() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // * 1.
    //  a new iterator that applies transformation to each element in the number
    let double = numbers.iter().map(|x| x * 2);
    for x in double {
        print!("{} ", x);
    }   // now we cannot use double, it is consumed
    println!();

    // * 2.
    let mut count = 0;
    for pair in vec!["Zeshan", "Ahmed", "Ali"].into_iter()
        .map(|single_name| {
            count += 1;
            (count, single_name)
        })
    {   // loop body starts here
        print!("{:?} ", pair);
    }
    // vector elements goes to single_name and returned back to pair(_,_)
}

fn enumerate_example() {
    let numbers = vec![2, 4, 6, 8];
    let even_indexes = numbers
        .iter() // reference to numbers
        .enumerate()    // returns (index,value) of type (usize,i32)
        .filter(|&(index, _)| index % 2 == 0) // returns Iterator to even indexes
        .map(|(_, value)| value); // as filter returned (index,value), we finally only want "values", so ignore indexes

    for even in even_indexes {
        print!("{} ", even);
    }
    println!();
}


fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    // * Uncomment the desired code to run
    into_iter();


    // loop_implementation();
    // map_examples();
    // enumerate_example()
}
