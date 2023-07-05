#[derive(Debug)]
enum Students{
    Name(String),
    Age(i32),
    Grade(char)
}

fn main() {
    // create vector with default values
    // let V = vec![1,5,7];

    // let mut float_values : Vec<f32> = vec![2.3,4.3];
    // float_values.push(5.8);

    let mut my_vector = Vec::new();     // created empty vector
    my_vector.push(5);
    my_vector.push(7);
    my_vector.push(4);

    for i in &my_vector{
        println!("{i}");
    }

    // creating vector of Student(enum) type
    let mut student_list = vec![
        Students::Name(String::from("Zeshan Khalid")),
        Students::Age(15),
        Students::Grade('A'),
        ];

    // student_list.push(Students::Age(4));

    let x= &stuent_list[1];
        println!("{}",x);

    // for i in &student_list{
    //     println!("{:?}",i);
    // }

}
