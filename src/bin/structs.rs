#[derive(Debug)]

struct Rectangle{
    height: u32,
    width: u32
}

fn calculate_area(rect : &Rectangle) ->u32{
    rect.height * rect.width
}

impl Rectangle {
    fn calculate_area(&self) -> u32{
        self.width * self.height
    }
    // can rect(self) hold rect(other)
    fn can_hold(&self, other:&Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}
fn main(){
    let plates = 5;
    let rect1 = Rectangle{
        width: 43,
        height : dbg!(plates * 5)
    };

    let rect2 = Rectangle{
        width : 30,
        height : 10
    };

    // println!("rect1 has -> {:#?}",rect1);
    dbg!(&rect1);

    // println!("Area is {}",calculate_area(&rect1));   // separate calculate_area function
    print!("Area is {}",rect1.calculate_area());    // after using [impl]

    print!("\nCan rect1 holds rect2 : {}",rect1.can_hold(&rect2));
}